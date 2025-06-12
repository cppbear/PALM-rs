type Bits = u32;
use exec::ProgramCache;
use input::{Input, InputAt};
use prog::{Program, InstPtr};
use re_trait::Slot;
const BIT_SIZE: usize = 32;
const MAX_SIZE_BYTES: usize = 256 * (1 << 10);
pub trait Input {
    fn at(&self, i: usize) -> InputAt;
    fn next_char(&self, at: InputAt) -> Char;
    fn previous_char(&self, at: InputAt) -> Char;
    fn is_empty_match(&self, at: InputAt, empty: &InstEmptyLook) -> bool;
    fn prefix_at(&self, prefixes: &LiteralSearcher, at: InputAt) -> Option<InputAt>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn as_bytes(&self) -> &[u8];
}
#[derive(Debug)]
pub struct Bounded<'a, 'm, 'r, 's, I> {
    prog: &'r Program,
    input: I,
    matches: &'m mut [bool],
    slots: &'s mut [Slot],
    m: &'a mut Cache,
}
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
#[derive(Clone, Debug)]
pub struct Cache {
    /// A pair of ordered sets for tracking NFA states.
    clist: Threads,
    nlist: Threads,
    /// An explicit stack used for following epsilon transitions.
    stack: Vec<FollowEpsilon>,
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
pub struct Cache {
    jobs: Vec<Job>,
    visited: Vec<Bits>,
}
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
impl<'a, 'm, 'r, 's, I: Input> Bounded<'a, 'm, 'r, 's, I> {
    pub fn exec(
        prog: &'r Program,
        cache: &ProgramCache,
        matches: &'m mut [bool],
        slots: &'s mut [Slot],
        input: I,
        start: usize,
    ) -> bool {}
    fn clear(&mut self) {}
    fn exec_(&mut self, mut at: InputAt) -> bool {}
    fn backtrack(&mut self, start: InputAt) -> bool {}
    fn step(&mut self, mut ip: InstPtr, mut at: InputAt) -> bool {}
    fn has_visited(&mut self, ip: InstPtr, at: InputAt) -> bool {
        let k = ip * (self.input.len() + 1) + at.pos();
        let k1 = k / BIT_SIZE;
        let k2 = usize_to_u32(1 << (k & (BIT_SIZE - 1)));
        if self.m.visited[k1] & k2 == 0 {
            self.m.visited[k1] |= k2;
            false
        } else {
            true
        }
    }
}
impl InputAt {
    pub fn is_start(&self) -> bool {}
    pub fn is_end(&self) -> bool {}
    pub fn char(&self) -> Char {}
    pub fn byte(&self) -> Option<u8> {}
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn next_pos(&self) -> usize {}
}
fn usize_to_u32(n: usize) -> u32 {
    if (n as u64) > (::std::u32::MAX as u64) {
        panic!("BUG: {} is too big to fit into u32", n)
    }
    n as u32
}
