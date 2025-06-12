use std::mem;
use exec::ProgramCache;
use input::{Input, InputAt};
use prog::{Program, InstPtr};
use re_trait::Slot;
use sparse::SparseSet;
#[derive(Clone, Debug)]
pub struct Cache {
    /// A pair of ordered sets for tracking NFA states.
    clist: Threads,
    nlist: Threads,
    /// An explicit stack used for following epsilon transitions.
    stack: Vec<FollowEpsilon>,
}
#[derive(Clone, Debug)]
struct Threads {
    /// An ordered set of opcodes (each opcode is an NFA state).
    set: SparseSet,
    /// Captures for every NFA state.
    ///
    /// It is stored in row-major order, where the columns are the capture
    /// slots and the rows are the states.
    caps: Vec<Slot>,
    /// The number of capture slots stored per thread. (Every capture has
    /// two slots.)
    slots_per_thread: usize,
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
enum FollowEpsilon {
    /// Follow transitions at the given instruction pointer.
    IP(InstPtr),
    /// Restore the capture slot with the given position in the input.
    Capture { slot: usize, pos: Slot },
}
impl Cache {
    pub fn new(_prog: &Program) -> Self {
        Cache {
            clist: Threads::new(),
            nlist: Threads::new(),
            stack: vec![],
        }
    }
}
impl Threads {
    fn new() -> Self {
        Threads {
            set: SparseSet::new(0),
            caps: vec![],
            slots_per_thread: 0,
        }
    }
    fn resize(&mut self, num_insts: usize, ncaps: usize) {}
    fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {}
}
