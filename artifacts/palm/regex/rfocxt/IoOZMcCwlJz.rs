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
#[derive(Clone, Debug)]
pub struct Cache {
    /// Group persistent DFA related cache state together. The sparse sets
    /// listed below are used as scratch space while computing uncached states.
    inner: CacheInner,
    /// qcur and qnext are ordered sets with constant time
    /// addition/membership/clearing-whole-set and linear time iteration. They
    /// are used to manage the sets of NFA states in DFA states when computing
    /// cached DFA states. In particular, the order of the NFA states matters
    /// for leftmost-first style matching. Namely, when computing a cached
    /// state, the set of NFA states stops growing as soon as the first Match
    /// instruction is observed.
    qcur: SparseSet,
    qnext: SparseSet,
}
#[derive(Clone)]
struct Transitions {
    /// The table.
    table: Vec<StatePtr>,
    /// The stride.
    num_byte_classes: usize,
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
#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    data: Box<[u8]>,
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
impl Cache {
    pub fn new(prog: &Program) -> Self {
        let num_byte_classes = (prog.byte_classes[255] as usize + 1) + 1;
        let starts = vec![STATE_UNKNOWN; 256];
        let mut cache = Cache {
            inner: CacheInner {
                compiled: HashMap::new(),
                trans: Transitions::new(num_byte_classes),
                states: vec![],
                start_states: starts,
                stack: vec![],
                flush_count: 0,
                size: 0,
            },
            qcur: SparseSet::new(prog.insts.len()),
            qnext: SparseSet::new(prog.insts.len()),
        };
        cache.inner.reset_size();
        cache
    }
}
impl Transitions {
    fn new(num_byte_classes: usize) -> Transitions {
        Transitions {
            table: vec![],
            num_byte_classes: num_byte_classes,
        }
    }
    fn num_states(&self) -> usize {}
    fn add(&mut self) -> Option<StatePtr> {}
    fn clear(&mut self) {}
    fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {}
    fn next(&self, si: StatePtr, cls: usize) -> StatePtr {}
    fn state_heap_size(&self) -> usize {}
    unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {}
}
impl SparseSet {
    pub fn new(size: usize) -> SparseSet {
        SparseSet {
            dense: vec![0; size],
            sparse: vec![0; size],
            size: 0,
        }
    }
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn capacity(&self) -> usize {}
    pub fn insert(&mut self, value: usize) {}
    pub fn contains(&self, value: usize) -> bool {}
    pub fn clear(&mut self) {}
}
impl CacheInner {
    fn reset_size(&mut self) {
        self.size = (self.start_states.len() * mem::size_of::<StatePtr>())
            + (self.stack.len() * mem::size_of::<InstPtr>());
    }
}
