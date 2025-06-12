type InstPtr = u32;
type StatePtr = u32;
use std::collections::HashMap;
use std::fmt;
use std::iter::repeat;
use std::mem;
use exec::ProgramCache;
use prog::{Inst, Program};
use sparse::SparseSet;
const STATE_UNKNOWN: StatePtr = 1 << 31;
const STATE_DEAD: StatePtr = STATE_UNKNOWN + 1;
const STATE_QUIT: StatePtr = STATE_DEAD + 1;
const STATE_START: StatePtr = 1 << 30;
const STATE_MATCH: StatePtr = 1 << 29;
const STATE_MAX: StatePtr = STATE_MATCH - 1;
#[derive(Debug)]
pub struct Fsm<'a> {
    /// prog contains the NFA instruction opcodes. DFA execution uses either
    /// the `dfa` instructions or the `dfa_reverse` instructions from
    /// `exec::ExecReadOnly`. (It never uses `ExecReadOnly.nfa`, which may have
    /// Unicode opcodes that cannot be executed by the DFA.)
    prog: &'a Program,
    /// The start state. We record it here because the pointer may change
    /// when the cache is wiped.
    start: StatePtr,
    /// The current position in the input.
    at: usize,
    /// Should we quit after seeing the first match? e.g., When the caller
    /// uses `is_match` or `shortest_match`.
    quit_after_match: bool,
    /// The last state that matched.
    ///
    /// When no match has occurred, this is set to STATE_UNKNOWN.
    ///
    /// This is only useful when matching regex sets. The last match state
    /// is useful because it contains all of the match instructions seen,
    /// thereby allowing us to enumerate which regexes in the set matched.
    last_match_si: StatePtr,
    /// The input position of the last cache flush. We use this to determine
    /// if we're thrashing in the cache too often. If so, the DFA quits so
    /// that we can fall back to the NFA algorithm.
    last_cache_flush: usize,
    /// All cached DFA information that is persisted between searches.
    cache: &'a mut CacheInner,
}
#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    data: Box<[u8]>,
}
#[derive(Copy, Clone, Debug)]
struct Byte(u16);
#[derive(Clone, Debug)]
pub struct SparseSet {
    /// Dense contains the instruction pointers in the order in which they
    /// were inserted. Accessing elements >= self.size is illegal.
    dense: Vec<usize>,
    /// Sparse maps instruction pointers to their location in dense.
    ///
    /// An instruction pointer is in the set if and only if
    /// sparse[ip] < size && ip == dense[sparse[ip]].
    sparse: Vec<usize>,
    /// The number of elements in the set.
    size: usize,
}
struct InstPtrs<'a> {
    base: usize,
    data: &'a [u8],
}
#[derive(Clone)]
pub struct Program {
    /// A sequence of instructions that represents an NFA.
    pub insts: Vec<Inst>,
    /// Pointers to each Match instruction in the sequence.
    ///
    /// This is always length 1 unless this program represents a regex set.
    pub matches: Vec<InstPtr>,
    /// The ordered sequence of all capture groups extracted from the AST.
    /// Unnamed groups are `None`.
    pub captures: Vec<Option<String>>,
    /// Pointers to all named capture groups into `captures`.
    pub capture_name_idx: Arc<HashMap<String, usize>>,
    /// A pointer to the start instruction. This can vary depending on how
    /// the program was compiled. For example, programs for use with the DFA
    /// engine have a `.*?` inserted at the beginning of unanchored regular
    /// expressions. The actual starting point of the program is after the
    /// `.*?`.
    pub start: InstPtr,
    /// A set of equivalence classes for discriminating bytes in the compiled
    /// program.
    pub byte_classes: Vec<u8>,
    /// When true, this program can only match valid UTF-8.
    pub only_utf8: bool,
    /// When true, this program uses byte range instructions instead of Unicode
    /// range instructions.
    pub is_bytes: bool,
    /// When true, the program is compiled for DFA matching. For example, this
    /// implies `is_bytes` and also inserts a preceding `.*?` for unanchored
    /// regexes.
    pub is_dfa: bool,
    /// When true, the program matches text in reverse (for use only in the
    /// DFA).
    pub is_reverse: bool,
    /// Whether the regex must match from the start of the input.
    pub is_anchored_start: bool,
    /// Whether the regex must match at the end of the input.
    pub is_anchored_end: bool,
    /// Whether this program contains a Unicode word boundary instruction.
    pub has_unicode_word_boundary: bool,
    /// A possibly empty machine for very quickly matching prefix literals.
    pub prefixes: LiteralSearcher,
    /// A limit on the size of the cache that the DFA is allowed to use while
    /// matching.
    ///
    /// The cache limit specifies approximately how much space we're willing to
    /// give to the state cache. Once the state cache exceeds the size, it is
    /// wiped and all states must be re-computed.
    ///
    /// Note that this value does not impact correctness. It can be set to 0
    /// and the DFA will run just fine. (It will only ever store exactly one
    /// state in the cache, and will likely run very slowly, but it will work.)
    ///
    /// Also note that this limit is *per thread of execution*. That is,
    /// if the same regex is used to search text across multiple threads
    /// simultaneously, then the DFA cache is not shared. Instead, copies are
    /// made.
    pub dfa_size_limit: usize,
}
#[derive(Clone, Debug)]
struct CacheInner {
    /// A cache of pre-compiled DFA states, keyed by the set of NFA states
    /// and the set of empty-width flags set at the byte in the input when the
    /// state was observed.
    ///
    /// A StatePtr is effectively a `*State`, but to avoid various inconvenient
    /// things, we just pass indexes around manually. The performance impact of
    /// this is probably an instruction or two in the inner loop. However, on
    /// 64 bit, each StatePtr is half the size of a *State.
    compiled: HashMap<State, StatePtr>,
    /// The transition table.
    ///
    /// The transition table is laid out in row-major order, where states are
    /// rows and the transitions for each state are columns. At a high level,
    /// given state `s` and byte `b`, the next state can be found at index
    /// `s * 256 + b`.
    ///
    /// This is, of course, a lie. A StatePtr is actually a pointer to the
    /// *start* of a row in this table. When indexing in the DFA's inner loop,
    /// this removes the need to multiply the StatePtr by the stride. Yes, it
    /// matters. This reduces the number of states we can store, but: the
    /// stride is rarely 256 since we define transitions in terms of
    /// *equivalence classes* of bytes. Each class corresponds to a set of
    /// bytes that never discriminate a distinct path through the DFA from each
    /// other.
    trans: Transitions,
    /// Our set of states. Note that `StatePtr / num_byte_classes` indexes
    /// this Vec rather than just a `StatePtr`.
    states: Vec<State>,
    /// A set of cached start states, which are limited to the number of
    /// permutations of flags set just before the initial byte of input. (The
    /// index into this vec is a `EmptyFlags`.)
    ///
    /// N.B. A start state can be "dead" (i.e., no possible match), so we
    /// represent it with a StatePtr.
    start_states: Vec<StatePtr>,
    /// Stack scratch space used to follow epsilon transitions in the NFA.
    /// (This permits us to avoid recursion.)
    ///
    /// The maximum stack size is the number of NFA states.
    stack: Vec<InstPtr>,
    /// The total number of times this cache has been flushed by the DFA
    /// because of space constraints.
    flush_count: u64,
    /// The total heap size of the DFA's cache. We use this to determine when
    /// we should flush the cache.
    size: usize,
}
#[derive(Clone, Debug)]
pub enum Result<T> {
    Match(T),
    NoMatch(usize),
    Quit,
}
impl<'a> Fsm<'a> {
    #[inline(always)]
    pub fn forward(
        prog: &'a Program,
        cache: &ProgramCache,
        quit_after_match: bool,
        text: &[u8],
        at: usize,
    ) -> Result<usize> {}
    #[inline(always)]
    pub fn reverse(
        prog: &'a Program,
        cache: &ProgramCache,
        quit_after_match: bool,
        text: &[u8],
        at: usize,
    ) -> Result<usize> {}
    #[inline(always)]
    pub fn forward_many(
        prog: &'a Program,
        cache: &ProgramCache,
        matches: &mut [bool],
        text: &[u8],
        at: usize,
    ) -> Result<usize> {}
    #[inline(always)]
    fn exec_at(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {
        debug_assert!(! self.prog.is_reverse);
        let mut result = Result::NoMatch(self.at);
        let (mut prev_si, mut next_si) = (self.start, self.start);
        let mut at = self.at;
        while at < text.len() {
            while next_si <= STATE_MAX && at < text.len() {
                prev_si = unsafe { self.next_si(next_si, text, at) };
                at += 1;
                if prev_si > STATE_MAX || at + 2 >= text.len() {
                    mem::swap(&mut prev_si, &mut next_si);
                    break;
                }
                next_si = unsafe { self.next_si(prev_si, text, at) };
                at += 1;
                if next_si > STATE_MAX {
                    break;
                }
                prev_si = unsafe { self.next_si(next_si, text, at) };
                at += 1;
                if prev_si > STATE_MAX {
                    mem::swap(&mut prev_si, &mut next_si);
                    break;
                }
                next_si = unsafe { self.next_si(prev_si, text, at) };
                at += 1;
            }
            if next_si & STATE_MATCH > 0 {
                next_si &= !STATE_MATCH;
                result = Result::Match(at - 1);
                if self.quit_after_match {
                    return result;
                }
                self.last_match_si = next_si;
                prev_si = next_si;
                if self.prog.matches.len() > 1 {
                    let state = self.state(next_si);
                    let just_matches = state
                        .inst_ptrs()
                        .all(|ip| self.prog[ip].is_match());
                    if just_matches {
                        return result;
                    }
                }
                let cur = at;
                while (next_si & !STATE_MATCH) == prev_si && at + 2 < text.len() {
                    next_si = unsafe { self.next_si(next_si & !STATE_MATCH, text, at) };
                    at += 1;
                }
                if at > cur {
                    result = Result::Match(at - 2);
                }
            } else if next_si & STATE_START > 0 {
                debug_assert!(self.has_prefix());
                next_si &= !STATE_START;
                prev_si = next_si;
                at = match self.prefix_at(text, at) {
                    None => return Result::NoMatch(text.len()),
                    Some(i) => i,
                };
            } else if next_si >= STATE_UNKNOWN {
                if next_si == STATE_QUIT {
                    return Result::Quit;
                }
                let byte = Byte::byte(text[at - 1]);
                prev_si &= STATE_MAX;
                self.at = at;
                next_si = match self.next_state(qcur, qnext, prev_si, byte) {
                    None => return Result::Quit,
                    Some(STATE_DEAD) => return result.set_non_match(at),
                    Some(si) => si,
                };
                debug_assert!(next_si != STATE_UNKNOWN);
                if next_si & STATE_MATCH > 0 {
                    next_si &= !STATE_MATCH;
                    result = Result::Match(at - 1);
                    if self.quit_after_match {
                        return result;
                    }
                    self.last_match_si = next_si;
                }
                prev_si = next_si;
            } else {
                prev_si = next_si;
            }
        }
        prev_si &= STATE_MAX;
        prev_si = match self.next_state(qcur, qnext, prev_si, Byte::eof()) {
            None => return Result::Quit,
            Some(STATE_DEAD) => return result.set_non_match(text.len()),
            Some(si) => si & !STATE_START,
        };
        debug_assert!(prev_si != STATE_UNKNOWN);
        if prev_si & STATE_MATCH > 0 {
            prev_si &= !STATE_MATCH;
            self.last_match_si = prev_si;
            result = Result::Match(text.len());
        }
        result
    }
    #[inline(always)]
    fn exec_at_reverse(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {}
    #[inline(always)]
    unsafe fn next_si(&self, si: StatePtr, text: &[u8], i: usize) -> StatePtr {
        debug_assert!(i < text.len());
        let b = *text.get_unchecked(i);
        debug_assert!((b as usize) < self.prog.byte_classes.len());
        let cls = *self.prog.byte_classes.get_unchecked(b as usize);
        self.cache.trans.next_unchecked(si, cls as usize)
    }
    fn exec_byte(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        mut si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {}
    fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {}
    fn cached_state(
        &mut self,
        q: &SparseSet,
        mut state_flags: StateFlags,
        current_state: Option<&mut StatePtr>,
    ) -> Option<StatePtr> {}
    fn cached_state_key(
        &mut self,
        q: &SparseSet,
        state_flags: &mut StateFlags,
    ) -> Option<State> {}
    fn clear_cache_and_save(&mut self, current_state: Option<&mut StatePtr>) -> bool {}
    fn clear_cache(&mut self) -> bool {}
    fn restore_state(&mut self, state: State) -> Option<StatePtr> {}
    fn next_state(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {
        if si == STATE_DEAD {
            return Some(STATE_DEAD);
        }
        match self.cache.trans.next(si, self.byte_class(b)) {
            STATE_UNKNOWN => self.exec_byte(qcur, qnext, si, b),
            STATE_QUIT => None,
            STATE_DEAD => Some(STATE_DEAD),
            nsi => Some(nsi),
        }
    }
    #[inline(always)]
    fn start_state(
        &mut self,
        q: &mut SparseSet,
        empty_flags: EmptyFlags,
        state_flags: StateFlags,
    ) -> Option<StatePtr> {}
    fn start_flags(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {}
    fn start_flags_reverse(&self, text: &[u8], at: usize) -> (EmptyFlags, StateFlags) {}
    fn state(&self, si: StatePtr) -> &State {
        &self.cache.states[si as usize / self.num_byte_classes()]
    }
    fn add_state(&mut self, state: State) -> Option<StatePtr> {}
    fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
        self.prog.prefixes.find(&text[at..]).map(|(s, _)| at + s)
    }
    fn num_byte_classes(&self) -> usize {}
    #[inline(always)]
    fn byte_class(&self, b: Byte) -> usize {}
    #[inline(always)]
    fn u8_class(&self, b: u8) -> usize {}
    fn continue_past_first_match(&self) -> bool {}
    fn has_prefix(&self) -> bool {
        !self.prog.is_reverse && !self.prog.prefixes.is_empty()
            && !self.prog.is_anchored_start
    }
    fn start_ptr(&self, si: StatePtr) -> StatePtr {}
    fn approximate_size(&self) -> usize {}
}
impl State {
    fn flags(&self) -> StateFlags {}
    fn inst_ptrs(&self) -> InstPtrs {
        InstPtrs {
            base: 0,
            data: &self.data[1..],
        }
    }
}
impl Byte {
    fn byte(b: u8) -> Self {
        Byte(b as u16)
    }
    fn eof() -> Self {
        Byte(256)
    }
    fn is_eof(&self) -> bool {}
    fn is_ascii_word(&self) -> bool {}
    fn as_byte(&self) -> Option<u8> {}
}
impl<T> Result<T> {
    pub fn is_match(&self) -> bool {}
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Result<U> {}
    fn set_non_match(self, at: usize) -> Result<T> {
        match self {
            Result::NoMatch(_) => Result::NoMatch(at),
            r => r,
        }
    }
}
