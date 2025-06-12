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
struct ByteClassSet([bool; 256]);
struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}
#[derive(Debug)]
struct Patch {
    hole: Hole,
    entry: InstPtr,
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
enum InstHole {
    Save { slot: usize },
    EmptyLook { look: EmptyLook },
    Char { c: char },
    Ranges { ranges: Vec<(char, char)> },
    Bytes { start: u8, end: u8 },
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
enum MaybeInst {
    Compiled(Inst),
    Uncompiled(InstHole),
    Split,
    Split1(InstPtr),
    Split2(InstPtr),
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
    fn compile_many(mut self, exprs: &[Hir]) -> result::Result<Program, Error> {}
    fn compile_finish(mut self) -> result::Result<Program, Error> {}
    fn c(&mut self, expr: &Hir) -> Result {}
    fn c_capture(&mut self, first_slot: usize, expr: &Hir) -> Result {}
    fn c_dotstar(&mut self) -> Result {}
    fn c_literal(&mut self, chars: &[char]) -> Result {}
    fn c_char(&mut self, c: char) -> Result {}
    fn c_class(&mut self, ranges: &[hir::ClassUnicodeRange]) -> Result {}
    fn c_bytes(&mut self, bytes: &[u8]) -> Result {}
    fn c_byte(&mut self, b: u8) -> Result {}
    fn c_class_bytes(&mut self, ranges: &[hir::ClassBytesRange]) -> Result {
        debug_assert!(! ranges.is_empty());
        let first_split_entry = self.insts.len();
        let mut holes = vec![];
        let mut prev_hole = Hole::None;
        for r in &ranges[0..ranges.len() - 1] {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let next = self.insts.len();
            self.byte_classes.set_range(r.start(), r.end());
            holes
                .push(
                    self
                        .push_hole(InstHole::Bytes {
                            start: r.start(),
                            end: r.end(),
                        }),
                );
            prev_hole = self.fill_split(split, Some(next), None);
        }
        let next = self.insts.len();
        let r = &ranges[ranges.len() - 1];
        self.byte_classes.set_range(r.start(), r.end());
        holes
            .push(
                self
                    .push_hole(InstHole::Bytes {
                        start: r.start(),
                        end: r.end(),
                    }),
            );
        self.fill(prev_hole, next);
        Ok(Patch {
            hole: Hole::Many(holes),
            entry: first_split_entry,
        })
    }
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
    fn push_compiled(&mut self, inst: Inst) {}
    fn push_hole(&mut self, inst: InstHole) -> Hole {
        let hole = self.insts.len();
        self.insts.push(MaybeInst::Uncompiled(inst));
        Hole::One(hole)
    }
    fn push_split_hole(&mut self) -> Hole {
        let hole = self.insts.len();
        self.insts.push(MaybeInst::Split);
        Hole::One(hole)
    }
    fn check_size(&self) -> result::Result<(), Error> {}
}
impl ByteClassSet {
    fn new() -> Self {
        ByteClassSet([false; 256])
    }
    fn set_range(&mut self, start: u8, end: u8) {
        debug_assert!(start <= end);
        if start > 0 {
            self.0[start as usize - 1] = true;
        }
        self.0[end as usize] = true;
    }
    fn set_word_boundary(&mut self) {}
    fn byte_classes(&self) -> Vec<u8> {}
}
