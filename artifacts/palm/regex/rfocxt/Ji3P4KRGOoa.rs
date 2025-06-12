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
struct ByteClassSet([bool; 256]);
#[derive(Debug)]
struct Patch {
    hole: Hole,
    entry: InstPtr,
}
struct SuffixCache {
    table: Vec<SuffixCacheEntry>,
    version: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyLook {
    /// Start of line or input.
    StartLine,
    /// End of line or input.
    EndLine,
    /// Start of input.
    StartText,
    /// End of input.
    EndText,
    /// Word character on one side and non-word character on other.
    WordBoundary,
    /// Word character on both sides or non-word character on both sides.
    NotWordBoundary,
    /// ASCII word boundary.
    WordBoundaryAscii,
    /// Not ASCII word boundary.
    NotWordBoundaryAscii,
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
#[derive(Debug)]
enum Hole {
    None,
    One(InstPtr),
    Many(Vec<Hole>),
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
    fn c_dotstar(&mut self) -> Result {}
    fn c_literal(&mut self, chars: &[char]) -> Result {
        debug_assert!(! chars.is_empty());
        let mut chars: Box<Iterator<Item = &char>> = if self.compiled.is_reverse {
            Box::new(chars.iter().rev())
        } else {
            Box::new(chars.iter())
        };
        let first = *chars.next().expect("non-empty literal");
        let Patch { mut hole, entry } = self.c_char(first)?;
        for &c in chars {
            let p = self.c_char(c)?;
            self.fill(hole, p.entry);
            hole = p.hole;
        }
        Ok(Patch { hole: hole, entry: entry })
    }
    fn c_char(&mut self, c: char) -> Result {}
    fn c_class(&mut self, ranges: &[hir::ClassUnicodeRange]) -> Result {
        assert!(! ranges.is_empty());
        if self.compiled.uses_bytes() {
            CompileClass {
                c: self,
                ranges: ranges,
            }
                .compile()
        } else {
            let ranges: Vec<(char, char)> = ranges
                .iter()
                .map(|r| (r.start(), r.end()))
                .collect();
            let hole = if ranges.len() == 1 && ranges[0].0 == ranges[0].1 {
                self.push_hole(InstHole::Char { c: ranges[0].0 })
            } else {
                self.push_hole(InstHole::Ranges { ranges: ranges })
            };
            Ok(Patch {
                hole: hole,
                entry: self.insts.len() - 1,
            })
        }
    }
    fn c_bytes(&mut self, bytes: &[u8]) -> Result {
        debug_assert!(! bytes.is_empty());
        let mut bytes: Box<Iterator<Item = &u8>> = if self.compiled.is_reverse {
            Box::new(bytes.iter().rev())
        } else {
            Box::new(bytes.iter())
        };
        let first = *bytes.next().expect("non-empty literal");
        let Patch { mut hole, entry } = self.c_byte(first)?;
        for &b in bytes {
            let p = self.c_byte(b)?;
            self.fill(hole, p.entry);
            hole = p.hole;
        }
        Ok(Patch { hole: hole, entry: entry })
    }
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
    fn c_empty_look(&mut self, look: EmptyLook) -> Result {
        let hole = self.push_hole(InstHole::EmptyLook { look: look });
        Ok(Patch {
            hole: hole,
            entry: self.insts.len() - 1,
        })
    }
    fn c_concat<'a, I>(&mut self, exprs: I) -> Result
    where
        I: IntoIterator<Item = &'a Hir>,
    {}
    fn c_alternate(&mut self, exprs: &[Hir]) -> Result {
        debug_assert!(exprs.len() >= 2, "alternates must have at least 2 exprs");
        let first_split_entry = self.insts.len();
        let mut holes = vec![];
        let mut prev_hole = Hole::None;
        for e in &exprs[0..exprs.len() - 1] {
            self.fill_to_next(prev_hole);
            let split = self.push_split_hole();
            let prev_entry = self.insts.len();
            let Patch { hole, entry } = self.c(e)?;
            if prev_entry == self.insts.len() {
                return Err(
                    Error::Syntax(
                        "alternations cannot currently contain \
                     empty sub-expressions"
                            .to_string(),
                    ),
                );
            }
            holes.push(hole);
            prev_hole = self.fill_split(split, Some(entry), None);
        }
        let prev_entry = self.insts.len();
        let Patch { hole, entry } = self.c(&exprs[exprs.len() - 1])?;
        if prev_entry == self.insts.len() {
            return Err(
                Error::Syntax(
                    "alternations cannot currently contain \
                 empty sub-expressions"
                        .to_string(),
                ),
            );
        }
        holes.push(hole);
        self.fill(prev_hole, entry);
        Ok(Patch {
            hole: Hole::Many(holes),
            entry: first_split_entry,
        })
    }
    fn c_repeat(&mut self, rep: &hir::Repetition) -> Result {
        use syntax::hir::RepetitionKind::*;
        match rep.kind {
            ZeroOrOne => self.c_repeat_zero_or_one(&rep.hir, rep.greedy),
            ZeroOrMore => self.c_repeat_zero_or_more(&rep.hir, rep.greedy),
            OneOrMore => self.c_repeat_one_or_more(&rep.hir, rep.greedy),
            Range(hir::RepetitionRange::Exactly(min_max)) => {
                self.c_repeat_range(&rep.hir, rep.greedy, min_max, min_max)
            }
            Range(hir::RepetitionRange::AtLeast(min)) => {
                self.c_repeat_range_min_or_more(&rep.hir, rep.greedy, min)
            }
            Range(hir::RepetitionRange::Bounded(min, max)) => {
                self.c_repeat_range(&rep.hir, rep.greedy, min, max)
            }
        }
    }
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
    fn fill(&mut self, hole: Hole, goto: InstPtr) {}
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
    fn check_size(&self) -> result::Result<(), Error> {
        use std::mem::size_of;
        if self.insts.len() * size_of::<Inst>() > self.size_limit {
            Err(Error::CompiledTooBig(self.size_limit))
        } else {
            Ok(())
        }
    }
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
    pub fn needs_dotstar(&self) -> bool {}
    pub fn uses_bytes(&self) -> bool {
        self.is_bytes || self.is_dfa
    }
    pub fn only_utf8(&self) -> bool {}
    pub fn approximate_size(&self) -> usize {}
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
    fn set_word_boundary(&mut self) {
        let iswb = is_word_byte;
        let mut b1: u16 = 0;
        let mut b2: u16;
        while b1 <= 255 {
            b2 = b1 + 1;
            while b2 <= 255 && iswb(b1 as u8) == iswb(b2 as u8) {
                b2 += 1;
            }
            self.set_range(b1 as u8, (b2 - 1) as u8);
            b1 = b2;
        }
    }
    fn byte_classes(&self) -> Vec<u8> {}
}
