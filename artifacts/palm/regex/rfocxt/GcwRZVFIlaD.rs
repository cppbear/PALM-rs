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
struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}
struct ByteClassSet([bool; 256]);
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
#[derive(Debug)]
enum Hole {
    None,
    One(InstPtr),
    Many(Vec<Hole>),
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
    fn c(&mut self, expr: &Hir) -> Result {
        use prog;
        use syntax::hir::HirKind::*;
        self.check_size()?;
        match *expr.kind() {
            Empty => {
                Ok(Patch {
                    hole: Hole::None,
                    entry: self.insts.len(),
                })
            }
            Literal(hir::Literal::Unicode(c)) => self.c_literal(&[c]),
            Literal(hir::Literal::Byte(b)) => {
                assert!(self.compiled.uses_bytes());
                self.c_bytes(&[b])
            }
            Class(hir::Class::Unicode(ref cls)) => self.c_class(cls.ranges()),
            Class(hir::Class::Bytes(ref cls)) => {
                if self.compiled.uses_bytes() {
                    self.c_class_bytes(cls.ranges())
                } else {
                    assert!(cls.is_all_ascii());
                    let mut char_ranges = vec![];
                    for r in cls.iter() {
                        let (s, e) = (r.start() as char, r.end() as char);
                        char_ranges.push(hir::ClassUnicodeRange::new(s, e));
                    }
                    self.c_class(&char_ranges)
                }
            }
            Anchor(hir::Anchor::StartLine) if self.compiled.is_reverse => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::EndLine)
            }
            Anchor(hir::Anchor::StartLine) => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::StartLine)
            }
            Anchor(hir::Anchor::EndLine) if self.compiled.is_reverse => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::StartLine)
            }
            Anchor(hir::Anchor::EndLine) => {
                self.byte_classes.set_range(b'\n', b'\n');
                self.c_empty_look(prog::EmptyLook::EndLine)
            }
            Anchor(hir::Anchor::StartText) if self.compiled.is_reverse => {
                self.c_empty_look(prog::EmptyLook::EndText)
            }
            Anchor(hir::Anchor::StartText) => {
                self.c_empty_look(prog::EmptyLook::StartText)
            }
            Anchor(hir::Anchor::EndText) if self.compiled.is_reverse => {
                self.c_empty_look(prog::EmptyLook::StartText)
            }
            Anchor(hir::Anchor::EndText) => self.c_empty_look(prog::EmptyLook::EndText),
            WordBoundary(hir::WordBoundary::Unicode) => {
                self.compiled.has_unicode_word_boundary = true;
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::WordBoundary)
            }
            WordBoundary(hir::WordBoundary::UnicodeNegate) => {
                self.compiled.has_unicode_word_boundary = true;
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::NotWordBoundary)
            }
            WordBoundary(hir::WordBoundary::Ascii) => {
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::WordBoundaryAscii)
            }
            WordBoundary(hir::WordBoundary::AsciiNegate) => {
                self.byte_classes.set_word_boundary();
                self.c_empty_look(prog::EmptyLook::NotWordBoundaryAscii)
            }
            Group(ref g) => {
                match g.kind {
                    hir::GroupKind::NonCapturing => self.c(&g.hir),
                    hir::GroupKind::CaptureIndex(index) => {
                        if index as usize >= self.compiled.captures.len() {
                            self.compiled.captures.push(None);
                        }
                        self.c_capture(2 * index as usize, &g.hir)
                    }
                    hir::GroupKind::CaptureName { index, ref name } => {
                        if index as usize >= self.compiled.captures.len() {
                            let n = name.to_string();
                            self.compiled.captures.push(Some(n.clone()));
                            self.capture_name_idx.insert(n, index as usize);
                        }
                        self.c_capture(2 * index as usize, &g.hir)
                    }
                }
            }
            Concat(ref es) => {
                if self.compiled.is_reverse {
                    self.c_concat(es.iter().rev())
                } else {
                    self.c_concat(es)
                }
            }
            Alternation(ref es) => self.c_alternate(&**es),
            Repetition(ref rep) => self.c_repeat(rep),
        }
    }
    fn c_capture(&mut self, first_slot: usize, expr: &Hir) -> Result {}
    fn c_dotstar(&mut self) -> Result {}
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
    {
        let mut exprs = exprs.into_iter();
        let first = match exprs.next() {
            Some(expr) => expr,
            None => {
                return Ok(Patch {
                    hole: Hole::None,
                    entry: self.insts.len(),
                });
            }
        };
        let Patch { mut hole, entry } = self.c(first)?;
        for e in exprs {
            let p = self.c(e)?;
            self.fill(hole, p.entry);
            hole = p.hole;
        }
        Ok(Patch { hole: hole, entry: entry })
    }
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
    fn fill_to_next(&mut self, hole: Hole) {}
    fn fill_split(
        &mut self,
        hole: Hole,
        goto1: Option<InstPtr>,
        goto2: Option<InstPtr>,
    ) -> Hole {}
    fn push_compiled(&mut self, inst: Inst) {}
    fn push_hole(&mut self, inst: InstHole) -> Hole {}
    fn push_split_hole(&mut self) -> Hole {}
    fn check_size(&self) -> result::Result<(), Error> {}
}
