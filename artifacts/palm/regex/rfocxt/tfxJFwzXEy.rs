use std::mem;
use exec::ProgramCache;
use input::{Input, InputAt};
use prog::{Program, InstPtr};
use re_trait::Slot;
use sparse::SparseSet;
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
pub struct Fsm<'r, I> {
    /// The sequence of opcodes (among other things) that is actually executed.
    ///
    /// The program may be byte oriented or Unicode codepoint oriented.
    prog: &'r Program,
    /// An explicit stack used for following epsilon transitions. (This is
    /// borrowed from the cache.)
    stack: &'r mut Vec<FollowEpsilon>,
    /// The input to search.
    input: I,
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
#[derive(Clone, Debug)]
pub struct ProgramCacheInner {
    pub pikevm: pikevm::Cache,
    pub backtrack: backtrack::Cache,
    pub dfa: dfa::Cache,
    pub dfa_reverse: dfa::Cache,
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
#[derive(Clone, Debug)]
enum FollowEpsilon {
    /// Follow transitions at the given instruction pointer.
    IP(InstPtr),
    /// Restore the capture slot with the given position in the input.
    Capture { slot: usize, pos: Slot },
}
impl<'r, I: Input> Fsm<'r, I> {
    pub fn exec(
        prog: &'r Program,
        cache: &ProgramCache,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        input: I,
        start: usize,
    ) -> bool {
        let mut cache = cache.borrow_mut();
        let cache = &mut cache.pikevm;
        cache.clist.resize(prog.len(), prog.captures.len());
        cache.nlist.resize(prog.len(), prog.captures.len());
        let at = input.at(start);
        Fsm {
            prog: prog,
            stack: &mut cache.stack,
            input: input,
        }
            .exec_(
                &mut cache.clist,
                &mut cache.nlist,
                matches,
                slots,
                quit_after_match,
                at,
            )
    }
    fn exec_(
        &mut self,
        mut clist: &mut Threads,
        mut nlist: &mut Threads,
        matches: &mut [bool],
        slots: &mut [Slot],
        quit_after_match: bool,
        mut at: InputAt,
    ) -> bool {
        let mut matched = false;
        let mut all_matched = false;
        clist.set.clear();
        nlist.set.clear();
        'LOOP: loop {
            if clist.set.is_empty() {
                if (matched && matches.len() <= 1) || all_matched
                    || (!at.is_start() && self.prog.is_anchored_start)
                {
                    break;
                }
                if !self.prog.prefixes.is_empty() {
                    at = match self.input.prefix_at(&self.prog.prefixes, at) {
                        None => break,
                        Some(at) => at,
                    };
                }
            }
            if clist.set.is_empty() || (!self.prog.is_anchored_start && !all_matched) {
                self.add(&mut clist, slots, 0, at);
            }
            let at_next = self.input.at(at.next_pos());
            for i in 0..clist.set.len() {
                let ip = clist.set[i];
                if self.step(&mut nlist, matches, slots, clist.caps(ip), ip, at, at_next)
                {
                    matched = true;
                    all_matched = all_matched || matches.iter().all(|&b| b);
                    if quit_after_match {
                        break 'LOOP;
                    }
                    if self.prog.matches.len() == 1 {
                        break;
                    }
                }
            }
            if at.is_end() {
                break;
            }
            at = at_next;
            mem::swap(clist, nlist);
            nlist.set.clear();
        }
        matched
    }
    fn step(
        &mut self,
        nlist: &mut Threads,
        matches: &mut [bool],
        slots: &mut [Slot],
        thread_caps: &mut [Option<usize>],
        ip: usize,
        at: InputAt,
        at_next: InputAt,
    ) -> bool {}
    fn add(
        &mut self,
        nlist: &mut Threads,
        thread_caps: &mut [Option<usize>],
        ip: usize,
        at: InputAt,
    ) {}
    fn add_step(
        &mut self,
        nlist: &mut Threads,
        thread_caps: &mut [Option<usize>],
        mut ip: usize,
        at: InputAt,
    ) {}
}
impl Deref for Program {
    type Target = [Inst];
    fn deref(&self) -> &Self::Target {
        &*self.insts
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
    fn resize(&mut self, num_insts: usize, ncaps: usize) {
        if num_insts == self.set.capacity() {
            return;
        }
        self.slots_per_thread = ncaps * 2;
        self.set = SparseSet::new(num_insts);
        self.caps = vec![None; self.slots_per_thread * num_insts];
    }
    fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {}
}
