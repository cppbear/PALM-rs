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
#[derive(Clone, Copy, Debug)]
pub struct InputAt {
    pos: usize,
    c: Char,
    byte: Option<u8>,
    len: usize,
}
#[derive(Clone, Debug)]
pub struct InstRanges {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The set of Unicode scalar value ranges to test.
    pub ranges: Vec<(char, char)>,
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
#[derive(Clone, Debug)]
pub struct InstSplit {
    /// The first instruction to try. A match resulting from following goto1
    /// has precedence over a match resulting from following goto2.
    pub goto1: InstPtr,
    /// The second instruction to try. A match resulting from following goto1
    /// has precedence over a match resulting from following goto2.
    pub goto2: InstPtr,
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
#[derive(Clone, Debug)]
pub struct InstSave {
    /// The next location to execute in the program.
    pub goto: InstPtr,
    /// The capture slot (there are two slots for every capture in a regex,
    /// including the zeroth capture for the entire match).
    pub slot: usize,
}
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Char(u32);
#[derive(Clone, Debug)]
pub struct Cache {
    /// A pair of ordered sets for tracking NFA states.
    clist: Threads,
    nlist: Threads,
    /// An explicit stack used for following epsilon transitions.
    stack: Vec<FollowEpsilon>,
}
#[derive(Clone, Debug)]
pub struct Cache {
    jobs: Vec<Job>,
    visited: Vec<Bits>,
}
#[derive(Clone, Debug)]
pub struct InstEmptyLook {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The type of zero-width assertion to check.
    pub look: EmptyLook,
}
#[derive(Clone, Debug)]
pub struct InstChar {
    /// The next location to execute in the program if this instruction
    /// succeeds.
    pub goto: InstPtr,
    /// The character to test.
    pub c: char,
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
#[derive(Clone, Copy, Debug)]
enum Job {
    Inst { ip: InstPtr, at: InputAt },
    SaveRestore { slot: usize, old_pos: Option<usize> },
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
    fn step(&mut self, mut ip: InstPtr, mut at: InputAt) -> bool {
        use prog::Inst::*;
        loop {
            if self.has_visited(ip, at) {
                return false;
            }
            match self.prog[ip] {
                Match(slot) => {
                    if slot < self.matches.len() {
                        self.matches[slot] = true;
                    }
                    return true;
                }
                Save(ref inst) => {
                    if let Some(&old_pos) = self.slots.get(inst.slot) {
                        self.m
                            .jobs
                            .push(Job::SaveRestore {
                                slot: inst.slot,
                                old_pos: old_pos,
                            });
                        self.slots[inst.slot] = Some(at.pos());
                    }
                    ip = inst.goto;
                }
                Split(ref inst) => {
                    self.m
                        .jobs
                        .push(Job::Inst {
                            ip: inst.goto2,
                            at: at,
                        });
                    ip = inst.goto1;
                }
                EmptyLook(ref inst) => {
                    if self.input.is_empty_match(at, inst) {
                        ip = inst.goto;
                    } else {
                        return false;
                    }
                }
                Char(ref inst) => {
                    if inst.c == at.char() {
                        ip = inst.goto;
                        at = self.input.at(at.next_pos());
                    } else {
                        return false;
                    }
                }
                Ranges(ref inst) => {
                    if inst.matches(at.char()) {
                        ip = inst.goto;
                        at = self.input.at(at.next_pos());
                    } else {
                        return false;
                    }
                }
                Bytes(ref inst) => {
                    if let Some(b) = at.byte() {
                        if inst.matches(b) {
                            ip = inst.goto;
                            at = self.input.at(at.next_pos());
                            continue;
                        }
                    }
                    return false;
                }
            }
        }
    }
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
impl Deref for Program {
    type Target = [Inst];
    fn deref(&self) -> &Self::Target {
        &*self.insts
    }
}
impl InputAt {
    pub fn is_start(&self) -> bool {}
    pub fn is_end(&self) -> bool {}
    pub fn char(&self) -> Char {
        self.c
    }
    pub fn byte(&self) -> Option<u8> {
        self.byte
    }
    pub fn len(&self) -> usize {}
    pub fn is_empty(&self) -> bool {}
    pub fn pos(&self) -> usize {
        self.pos
    }
    pub fn next_pos(&self) -> usize {
        self.pos + self.len
    }
}
impl InstRanges {
    pub fn matches(&self, c: Char) -> bool {
        for r in self.ranges.iter().take(4) {
            if c < r.0 {
                return false;
            }
            if c <= r.1 {
                return true;
            }
        }
        self.ranges
            .binary_search_by(|r| {
                if r.1 < c {
                    Ordering::Less
                } else if r.0 > c {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .is_ok()
    }
    pub fn num_chars(&self) -> usize {}
}
impl InstBytes {
    pub fn matches(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
}
