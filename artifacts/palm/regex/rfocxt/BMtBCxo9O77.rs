type Result = result::Result<Patch, Error>;
use std::collections::HashMap;
use std::iter;
use std::result;
use std::sync::Arc;
use syntax::is_word_byte;
use syntax::hir::{self, Hir};
use utf8_ranges::{Utf8Range, Utf8Sequence, Utf8Sequences};
use prog::{
    Program, Inst, InstPtr, EmptyLook, InstSave, InstSplit, InstEmptyLook, InstChar,
    InstRanges, InstBytes,
};
use Error;
pub struct Compiler {
    insts: Vec<MaybeInst>,
    compiled: Program,
    capture_name_idx: HashMap<String, usize>,
    num_exprs: usize,
    size_limit: usize,
    suffix_cache: SuffixCache,
    utf8_seqs: Option<Utf8Sequences>,
    byte_classes: ByteClassSet,
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
struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}
#[derive(Debug)]
struct Patch {
    hole: Hole,
    entry: InstPtr,
}
struct ByteClassSet([bool; 256]);
#[derive(Clone, Debug)]
enum MaybeInst {
    Compiled(Inst),
    Uncompiled(InstHole),
    Split,
    Split1(InstPtr),
    Split2(InstPtr),
}
#[derive(Clone, PartialEq)]
pub enum Error {
    /// A syntax error.
    Syntax(String),
    /// The compiled program exceeded the set size limit.
    /// The argument is the size limit imposed.
    CompiledTooBig(usize),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Debug)]
enum Hole {
    None,
    One(InstPtr),
    Many(Vec<Hole>),
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
impl Compiler {
    pub fn new() -> Self {
        Compiler {
            insts: vec![],
            compiled: Program::new(),
            capture_name_idx: HashMap::new(),
            num_exprs: 0,
            size_limit: 10 * (1 << 20),
            suffix_cache: SuffixCache::new(1000),
            utf8_seqs: Some(Utf8Sequences::new('\x00', '\x00')),
            byte_classes: ByteClassSet::new(),
        }
    }
    pub fn size_limit(mut self, size_limit: usize) -> Self {
        self.size_limit = size_limit;
        self
    }
    pub fn bytes(mut self, yes: bool) -> Self {
        self.compiled.is_bytes = yes;
        self
    }
    pub fn only_utf8(mut self, yes: bool) -> Self {
        self.compiled.only_utf8 = yes;
        self
    }
    pub fn dfa(mut self, yes: bool) -> Self {
        self.compiled.is_dfa = yes;
        self
    }
    pub fn reverse(mut self, yes: bool) -> Self {
        self.compiled.is_reverse = yes;
        self
    }
    pub fn compile(mut self, exprs: &[Hir]) -> result::Result<Program, Error> {}
    fn compile_one(mut self, expr: &Hir) -> result::Result<Program, Error> {}
    fn compile_many(mut self, exprs: &[Hir]) -> result::Result<Program, Error> {
        debug_assert!(exprs.len() > 1);
        self.compiled.is_anchored_start = exprs.iter().all(|e| e.is_anchored_start());
        self.compiled.is_anchored_end = exprs.iter().all(|e| e.is_anchored_end());
        let mut dotstar_patch = Patch {
            hole: Hole::None,
            entry: 0,
        };
        if self.compiled.needs_dotstar() {
            dotstar_patch = self.c_dotstar()?;
            self.compiled.start = dotstar_patch.entry;
        } else {
            self.compiled.start = 0;
        }
        self.fill_to_next(dotstar_patch.hole);
        let mut prev_hole = Hole::None;
        for (i, expr) in exprs[0..exprs.len() - 1].iter().enumerate() {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let Patch { hole, entry } = self.c_capture(0, expr)?;
            self.fill_to_next(hole);
            self.compiled.matches.push(self.insts.len());
            self.push_compiled(Inst::Match(i));
            prev_hole = self.fill_split(split, Some(entry), None);
        }
        let i = exprs.len() - 1;
        let Patch { hole, entry } = self.c_capture(0, &exprs[i])?;
        self.fill(prev_hole, entry);
        self.fill_to_next(hole);
        self.compiled.matches.push(self.insts.len());
        self.push_compiled(Inst::Match(i));
        self.compile_finish()
    }
    fn compile_finish(mut self) -> result::Result<Program, Error> {
        self.compiled.insts = self.insts.into_iter().map(|inst| inst.unwrap()).collect();
        self.compiled.byte_classes = self.byte_classes.byte_classes();
        self.compiled.capture_name_idx = Arc::new(self.capture_name_idx);
        Ok(self.compiled)
    }
    fn c(&mut self, expr: &Hir) -> Result {}
    fn c_capture(&mut self, first_slot: usize, expr: &Hir) -> Result {
        if self.num_exprs > 1 || self.compiled.is_dfa {
            self.c(expr)
        } else {
            let entry = self.insts.len();
            let hole = self.push_hole(InstHole::Save { slot: first_slot });
            let patch = self.c(expr)?;
            self.fill(hole, patch.entry);
            self.fill_to_next(patch.hole);
            let hole = self
                .push_hole(InstHole::Save {
                    slot: first_slot + 1,
                });
            Ok(Patch { hole: hole, entry: entry })
        }
    }
    fn c_dotstar(&mut self) -> Result {
        Ok(
            if !self.compiled.only_utf8() {
                self.c(
                    &Hir::repetition(hir::Repetition {
                        kind: hir::RepetitionKind::ZeroOrMore,
                        greedy: false,
                        hir: Box::new(Hir::any(true)),
                    }),
                )?
            } else {
                self.c(
                    &Hir::repetition(hir::Repetition {
                        kind: hir::RepetitionKind::ZeroOrMore,
                        greedy: false,
                        hir: Box::new(Hir::any(false)),
                    }),
                )?
            },
        )
    }
    fn c_literal(&mut self, chars: &[char]) -> Result {}
    fn c_char(&mut self, c: char) -> Result {}
    fn c_class(&mut self, ranges: &[hir::ClassUnicodeRange]) -> Result {}
    fn c_bytes(&mut self, bytes: &[u8]) -> Result {}
    fn c_byte(&mut self, b: u8) -> Result {}
    fn c_class_bytes(&mut self, ranges: &[hir::ClassBytesRange]) -> Result {}
    fn c_empty_look(&mut self, look: EmptyLook) -> Result {}
    fn c_concat<'a, I>(&mut self, exprs: I) -> Result
    where
        I: IntoIterator<Item = &'a Hir>,
    {}
    fn c_alternate(&mut self, exprs: &[Hir]) -> Result {}
    fn c_repeat(&mut self, rep: &hir::Repetition) -> Result {}
    fn c_repeat_zero_or_one(&mut self, expr: &Hir, greedy: bool) -> Result {}
    fn c_repeat_zero_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {}
    fn c_repeat_one_or_more(&mut self, expr: &Hir, greedy: bool) -> Result {}
    fn c_repeat_range_min_or_more(
        &mut self,
        expr: &Hir,
        greedy: bool,
        min: u32,
    ) -> Result {}
    fn c_repeat_range(
        &mut self,
        expr: &Hir,
        greedy: bool,
        min: u32,
        max: u32,
    ) -> Result {}
    fn fill(&mut self, hole: Hole, goto: InstPtr) {
        match hole {
            Hole::None => {}
            Hole::One(pc) => {
                self.insts[pc].fill(goto);
            }
            Hole::Many(holes) => {
                for hole in holes {
                    self.fill(hole, goto);
                }
            }
        }
    }
    fn fill_to_next(&mut self, hole: Hole) {
        let next = self.insts.len();
        self.fill(hole, next);
    }
    fn fill_split(
        &mut self,
        hole: Hole,
        goto1: Option<InstPtr>,
        goto2: Option<InstPtr>,
    ) -> Hole {
        match hole {
            Hole::None => Hole::None,
            Hole::One(pc) => {
                match (goto1, goto2) {
                    (Some(goto1), Some(goto2)) => {
                        self.insts[pc].fill_split(goto1, goto2);
                        Hole::None
                    }
                    (Some(goto1), None) => {
                        self.insts[pc].half_fill_split_goto1(goto1);
                        Hole::One(pc)
                    }
                    (None, Some(goto2)) => {
                        self.insts[pc].half_fill_split_goto2(goto2);
                        Hole::One(pc)
                    }
                    (None, None) => {
                        unreachable!(
                            "at least one of the split \
                                                  holes must be filled"
                        )
                    }
                }
            }
            Hole::Many(holes) => {
                let mut new_holes = vec![];
                for hole in holes {
                    new_holes.push(self.fill_split(hole, goto1, goto2));
                }
                if new_holes.is_empty() {
                    Hole::None
                } else if new_holes.len() == 1 {
                    new_holes.pop().unwrap()
                } else {
                    Hole::Many(new_holes)
                }
            }
        }
    }
    fn push_compiled(&mut self, inst: Inst) {
        self.insts.push(MaybeInst::Compiled(inst));
    }
    fn push_hole(&mut self, inst: InstHole) -> Hole {}
    fn push_split_hole(&mut self) -> Hole {
        let hole = self.insts.len();
        self.insts.push(MaybeInst::Split);
        Hole::One(hole)
    }
    fn check_size(&self) -> result::Result<(), Error> {}
}
impl Program {
    pub fn new() -> Self {
        Program {
            insts: vec![],
            matches: vec![],
            captures: vec![],
            capture_name_idx: Arc::new(HashMap::new()),
            start: 0,
            byte_classes: vec![0; 256],
            only_utf8: true,
            is_bytes: false,
            is_dfa: false,
            is_reverse: false,
            is_anchored_start: false,
            is_anchored_end: false,
            has_unicode_word_boundary: false,
            prefixes: LiteralSearcher::empty(),
            dfa_size_limit: 2 * (1 << 20),
        }
    }
    pub fn skip(&self, mut pc: usize) -> usize {}
    pub fn leads_to_match(&self, pc: usize) -> bool {}
    pub fn needs_dotstar(&self) -> bool {
        self.is_dfa && !self.is_reverse && !self.is_anchored_start
    }
    pub fn uses_bytes(&self) -> bool {}
    pub fn only_utf8(&self) -> bool {}
    pub fn approximate_size(&self) -> usize {}
}
