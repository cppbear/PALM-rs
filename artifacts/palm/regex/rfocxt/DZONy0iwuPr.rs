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
#[derive(Clone, Copy, Eq, Default, Hash, PartialEq)]
struct StateFlags(u8);
#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    data: Box<[u8]>,
}
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
#[derive(Copy, Clone, Debug)]
struct Byte(u16);
#[derive(Clone)]
struct Transitions {
    /// The table.
    table: Vec<StatePtr>,
    /// The stride.
    num_byte_classes: usize,
}
#[derive(Clone, Debug)]
pub struct InstBytes {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The start (inclusive) of this byte range.
    pub start: u8,
    /// The end (inclusive) of this byte range.
    pub end: u8,
}
struct InstPtrs<'a> {
    base: usize,
    data: &'a [u8],
}
#[derive(Clone, Copy, Eq, Debug, Default, Hash, PartialEq)]
struct EmptyFlags {
    start: bool,
    end: bool,
    start_line: bool,
    end_line: bool,
    word_boundary: bool,
    not_word_boundary: bool,
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
pub enum Inst {
    /// Match indicates that the program has reached a match state.
    ///
    /// The number in the match corresponds to the Nth logical regular
    /// expression in this program. This index is always 0 for normal regex
    /// programs. Values greater than 0 appear when compiling regex sets, and
    /// each match instruction gets its own unique value. The value corresponds
    /// to the Nth regex in the set.
    Match(usize),
    /// Save causes the program to save the current location of the input in
    /// the slot indicated by InstSave.
    Save(InstSave),
    /// Split causes the program to diverge to one of two paths in the
    /// program, preferring goto1 in InstSplit.
    Split(InstSplit),
    /// EmptyLook represents a zero-width assertion in a regex program. A
    /// zero-width assertion does not consume any of the input text.
    EmptyLook(InstEmptyLook),
    /// Char requires the regex program to match the character in InstChar at
    /// the current position in the input.
    Char(InstChar),
    /// Ranges requires the regex program to match the character at the current
    /// position in the input with one of the ranges specified in InstRanges.
    Ranges(InstRanges),
    /// Bytes is like Ranges, except it expresses a single byte range. It is
    /// used in conjunction with Split instructions to implement multi-byte
    /// character classes.
    Bytes(InstBytes),
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
    ) -> Result<usize> {}
    #[inline(always)]
    fn exec_at_reverse(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        text: &[u8],
    ) -> Result<usize> {}
    #[inline(always)]
    unsafe fn next_si(&self, si: StatePtr, text: &[u8], i: usize) -> StatePtr {}
    fn exec_byte(
        &mut self,
        qcur: &mut SparseSet,
        qnext: &mut SparseSet,
        mut si: StatePtr,
        b: Byte,
    ) -> Option<StatePtr> {
        use prog::Inst::*;
        qcur.clear();
        for ip in self.state(si).inst_ptrs() {
            qcur.insert(ip);
        }
        let is_word_last = self.state(si).flags().is_word();
        let is_word = b.is_ascii_word();
        if self.state(si).flags().has_empty() {
            let mut flags = EmptyFlags::default();
            if b.is_eof() {
                flags.end = true;
                flags.end_line = true;
            } else if b.as_byte().map_or(false, |b| b == b'\n') {
                flags.end_line = true;
            }
            if is_word_last == is_word {
                flags.not_word_boundary = true;
            } else {
                flags.word_boundary = true;
            }
            qnext.clear();
            for &ip in &*qcur {
                self.follow_epsilons(usize_to_u32(ip), qnext, flags);
            }
            mem::swap(qcur, qnext);
        }
        let mut empty_flags = EmptyFlags::default();
        let mut state_flags = StateFlags::default();
        empty_flags.start_line = b.as_byte().map_or(false, |b| b == b'\n');
        if b.is_ascii_word() {
            state_flags.set_word();
        }
        qnext.clear();
        for &ip in &*qcur {
            match self.prog[ip as usize] {
                Char(_) | Ranges(_) => unreachable!(),
                Save(_) | Split(_) | EmptyLook(_) => {}
                Match(_) => {
                    state_flags.set_match();
                    if !self.continue_past_first_match() {
                        break;
                    } else if self.prog.matches.len() > 1 && !qnext.contains(ip as usize)
                    {
                        qnext.insert(ip);
                    }
                }
                Bytes(ref inst) => {
                    if b.as_byte().map_or(false, |b| inst.matches(b)) {
                        self.follow_epsilons(inst.goto as InstPtr, qnext, empty_flags);
                    }
                }
            }
        }
        let cache = if b.is_eof() && self.prog.matches.len() > 1 {
            mem::swap(qcur, qnext);
            false
        } else {
            true
        };
        let mut next = match self.cached_state(qnext, state_flags, Some(&mut si)) {
            None => return None,
            Some(next) => next,
        };
        if (self.start & !STATE_START) == next {
            debug_assert!(! self.state(next).flags().is_match());
            next = self.start_ptr(next);
        }
        if next <= STATE_MAX && self.state(next).flags().is_match() {
            next |= STATE_MATCH;
        }
        debug_assert!(next != STATE_UNKNOWN);
        if cache {
            let cls = self.byte_class(b);
            self.cache.trans.set_next(si, cls, next);
        }
        Some(next)
    }
    fn follow_epsilons(&mut self, ip: InstPtr, q: &mut SparseSet, flags: EmptyFlags) {
        use prog::Inst::*;
        use prog::EmptyLook::*;
        self.cache.stack.push(ip);
        while let Some(mut ip) = self.cache.stack.pop() {
            loop {
                if q.contains(ip as usize) {
                    break;
                }
                q.insert(ip as usize);
                match self.prog[ip as usize] {
                    Char(_) | Ranges(_) => unreachable!(),
                    Match(_) | Bytes(_) => {
                        break;
                    }
                    EmptyLook(ref inst) => {
                        match inst.look {
                            StartLine if flags.start_line => {
                                ip = inst.goto as InstPtr;
                            }
                            EndLine if flags.end_line => {
                                ip = inst.goto as InstPtr;
                            }
                            StartText if flags.start => {
                                ip = inst.goto as InstPtr;
                            }
                            EndText if flags.end => {
                                ip = inst.goto as InstPtr;
                            }
                            WordBoundaryAscii if flags.word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            NotWordBoundaryAscii if flags.not_word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            WordBoundary if flags.word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            NotWordBoundary if flags.not_word_boundary => {
                                ip = inst.goto as InstPtr;
                            }
                            StartLine
                            | EndLine
                            | StartText
                            | EndText
                            | WordBoundaryAscii
                            | NotWordBoundaryAscii
                            | WordBoundary
                            | NotWordBoundary => {
                                break;
                            }
                        }
                    }
                    Save(ref inst) => {
                        ip = inst.goto as InstPtr;
                    }
                    Split(ref inst) => {
                        self.cache.stack.push(inst.goto2 as InstPtr);
                        ip = inst.goto1 as InstPtr;
                    }
                }
            }
        }
    }
    fn cached_state(
        &mut self,
        q: &SparseSet,
        mut state_flags: StateFlags,
        current_state: Option<&mut StatePtr>,
    ) -> Option<StatePtr> {
        let key = match self.cached_state_key(q, &mut state_flags) {
            None => return Some(STATE_DEAD),
            Some(v) => v,
        };
        if let Some(&si) = self.cache.compiled.get(&key) {
            return Some(si);
        }
        if self.approximate_size() > self.prog.dfa_size_limit
            && !self.clear_cache_and_save(current_state)
        {
            return None;
        }
        self.add_state(key)
    }
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
    ) -> Option<StatePtr> {}
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
    fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {}
    fn num_byte_classes(&self) -> usize {}
    #[inline(always)]
    fn byte_class(&self, b: Byte) -> usize {
        match b.as_byte() {
            None => self.num_byte_classes() - 1,
            Some(b) => self.u8_class(b),
        }
    }
    #[inline(always)]
    fn u8_class(&self, b: u8) -> usize {}
    fn continue_past_first_match(&self) -> bool {
        self.prog.is_reverse || self.prog.matches.len() > 1
    }
    fn has_prefix(&self) -> bool {}
    fn start_ptr(&self, si: StatePtr) -> StatePtr {
        if self.has_prefix() { si | STATE_START } else { si }
    }
    fn approximate_size(&self) -> usize {}
}
impl Deref for Program {
    type Target = [Inst];
    fn deref(&self) -> &Self::Target {
        &*self.insts
    }
}
impl StateFlags {
    fn is_match(&self) -> bool {
        self.0 & 0b0000000_1 > 0
    }
    fn set_match(&mut self) {
        self.0 |= 0b0000000_1;
    }
    fn is_word(&self) -> bool {
        self.0 & 0b000000_1_0 > 0
    }
    fn set_word(&mut self) {
        self.0 |= 0b000000_1_0;
    }
    fn has_empty(&self) -> bool {
        self.0 & 0b00000_1_00 > 0
    }
    fn set_empty(&mut self) {}
}
impl State {
    fn flags(&self) -> StateFlags {
        StateFlags(self.data[0])
    }
    fn inst_ptrs(&self) -> InstPtrs {
        InstPtrs {
            base: 0,
            data: &self.data[1..],
        }
    }
}
impl SparseSet {
    pub fn new(size: usize) -> SparseSet {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn capacity(&self) -> usize {}
    pub fn insert(&mut self, value: usize) {
        let i = self.size;
        self.dense[i] = value;
        self.sparse[value] = i;
        self.size += 1;
    }
    pub fn contains(&self, value: usize) -> bool {
        let i = self.sparse[value];
        i < self.size && self.dense[i] == value
    }
    pub fn clear(&mut self) {
        self.size = 0;
    }
}
impl Byte {
    fn byte(b: u8) -> Self {
        Byte(b as u16)
    }
    fn eof() -> Self {
        Byte(256)
    }
    fn is_eof(&self) -> bool {
        self.0 == 256
    }
    fn is_ascii_word(&self) -> bool {
        let b = match self.as_byte() {
            None => return false,
            Some(b) => b,
        };
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' => true,
            _ => false,
        }
    }
    fn as_byte(&self) -> Option<u8> {
        if self.is_eof() { None } else { Some(self.0 as u8) }
    }
}
impl Transitions {
    fn new(num_byte_classes: usize) -> Transitions {}
    fn num_states(&self) -> usize {}
    fn add(&mut self) -> Option<StatePtr> {}
    fn clear(&mut self) {}
    fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {
        self.table[si as usize + cls] = next;
    }
    fn next(&self, si: StatePtr, cls: usize) -> StatePtr {}
    fn state_heap_size(&self) -> usize {}
    unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {}
}
fn usize_to_u32(n: usize) -> u32 {
    if (n as u64) > (::std::u32::MAX as u64) {
        panic!("BUG: {} is too big to fit into u32", n)
    }
    n as u32
}
