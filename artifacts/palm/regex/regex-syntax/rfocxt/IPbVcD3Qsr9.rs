use std::fmt;
use hir::{self, Hir, HirKind};
use hir::visitor::{self, Visitor};
use is_meta_character;
pub trait Visitor {
    type Output;
    type Err;
    fn finish(self) -> Result<Self::Output, Self::Err>;
    fn start(&mut self);
    fn visit_pre(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_post(&mut self, _hir: &Hir) -> Result<(), Self::Err> {
        Ok(())
    }
    fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
        Ok(())
    }
}
#[derive(Debug)]
struct Writer<'p, W> {
    printer: &'p mut Printer,
    wtr: W,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassBytesRange {
    start: u8,
    end: u8,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ClassUnicodeRange {
    start: char,
    end: char,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassUnicode {
    set: IntervalSet<ClassUnicodeRange>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClassBytes {
    set: IntervalSet<ClassBytesRange>,
}
#[derive(Debug)]
pub struct ClassUnicodeIter<'a>(IntervalSetIter<'a, ClassUnicodeRange>);
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the parser generated the error from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error.
    span: Span,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Group {
    /// The kind of this group. If it is a capturing group, then the kind
    /// contains the capture group index (and the name, if it is a named
    /// group).
    pub kind: GroupKind,
    /// The expression inside the capturing group, which may be empty.
    pub hir: Box<Hir>,
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Debug)]
pub struct ClassBytesIter<'a>(IntervalSetIter<'a, ClassBytesRange>);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    /// The kind of error.
    kind: ErrorKind,
    /// The original pattern that the translator's Ast was parsed from. Every
    /// span in an error is a valid range into this string.
    pattern: String,
    /// The span of this error, derived from the Ast given to the translator.
    span: Span,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// An error that occurred while translating concrete syntax into abstract
    /// syntax (AST).
    Parse(ast::Error),
    /// An error that occurred while translating abstract syntax into a high
    /// level intermediate representation (HIR).
    Translate(hir::Error),
    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HirKind {
    /// The empty regular expression, which matches everything, including the
    /// empty string.
    Empty,
    /// A single literal character that matches exactly this character.
    Literal(Literal),
    /// A single character class that matches any of the characters in the
    /// class. A class can either consist of Unicode scalar values as
    /// characters, or it can use bytes.
    Class(Class),
    /// An anchor assertion. An anchor assertion match always has zero length.
    Anchor(Anchor),
    /// A word boundary assertion, which may or may not be Unicode aware. A
    /// word boundary assertion match always has zero length.
    WordBoundary(WordBoundary),
    /// A repetition operation applied to a child expression.
    Repetition(Repetition),
    /// A possibly capturing group, which contains a child expression.
    Group(Group),
    /// A concatenation of expressions. A concatenation always has at least two
    /// child expressions.
    ///
    /// A concatenation matches only if each of its child expression matches
    /// one after the other.
    Concat(Vec<Hir>),
    /// An alternation of expressions. An alternation always has at least two
    /// child expressions.
    ///
    /// An alternation matches only if at least one of its child expression
    /// matches. If multiple expressions match, then the leftmost is preferred.
    Alternation(Vec<Hir>),
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
}
impl<'p, W: fmt::Write> Visitor for Writer<'p, W> {
    type Output = ();
    type Err = fmt::Error;
    fn finish(self) -> fmt::Result {}
    fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {
        match *hir.kind() {
            HirKind::Empty
            | HirKind::Repetition(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => {}
            HirKind::Literal(hir::Literal::Unicode(c)) => {
                self.write_literal_char(c)?;
            }
            HirKind::Literal(hir::Literal::Byte(b)) => {
                self.write_literal_byte(b)?;
            }
            HirKind::Class(hir::Class::Unicode(ref cls)) => {
                self.wtr.write_str("[")?;
                for range in cls.iter() {
                    if range.start() == range.end() {
                        self.write_literal_char(range.start())?;
                    } else {
                        self.write_literal_char(range.start())?;
                        self.wtr.write_str("-")?;
                        self.write_literal_char(range.end())?;
                    }
                }
                self.wtr.write_str("]")?;
            }
            HirKind::Class(hir::Class::Bytes(ref cls)) => {
                self.wtr.write_str("(?-u:[")?;
                for range in cls.iter() {
                    if range.start() == range.end() {
                        self.write_literal_class_byte(range.start())?;
                    } else {
                        self.write_literal_class_byte(range.start())?;
                        self.wtr.write_str("-")?;
                        self.write_literal_class_byte(range.end())?;
                    }
                }
                self.wtr.write_str("])")?;
            }
            HirKind::Anchor(hir::Anchor::StartLine) => {
                self.wtr.write_str("(?m:^)")?;
            }
            HirKind::Anchor(hir::Anchor::EndLine) => {
                self.wtr.write_str("(?m:$)")?;
            }
            HirKind::Anchor(hir::Anchor::StartText) => {
                self.wtr.write_str(r"\A")?;
            }
            HirKind::Anchor(hir::Anchor::EndText) => {
                self.wtr.write_str(r"\z")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::Unicode) => {
                self.wtr.write_str(r"\b")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::UnicodeNegate) => {
                self.wtr.write_str(r"\B")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::Ascii) => {
                self.wtr.write_str(r"(?-u:\b)")?;
            }
            HirKind::WordBoundary(hir::WordBoundary::AsciiNegate) => {
                self.wtr.write_str(r"(?-u:\B)")?;
            }
            HirKind::Group(ref x) => {
                match x.kind {
                    hir::GroupKind::CaptureIndex(_) => {
                        self.wtr.write_str("(")?;
                    }
                    hir::GroupKind::CaptureName { ref name, .. } => {
                        write!(self.wtr, "(?P<{}>", name)?;
                    }
                    hir::GroupKind::NonCapturing => {
                        self.wtr.write_str("(?:")?;
                    }
                }
            }
        }
        Ok(())
    }
    fn visit_post(&mut self, hir: &Hir) -> fmt::Result {}
    fn visit_alternation_in(&mut self) -> fmt::Result {}
}
impl ClassBytesRange {
    pub fn new(start: u8, end: u8) -> ClassBytesRange {}
    pub fn start(&self) -> u8 {
        self.start
    }
    pub fn end(&self) -> u8 {
        self.end
    }
}
impl<'p, W: fmt::Write> Writer<'p, W> {
    fn write_literal_char(&mut self, c: char) -> fmt::Result {
        if is_meta_character(c) {
            self.wtr.write_str("\\")?;
        }
        self.wtr.write_char(c)
    }
    fn write_literal_byte(&mut self, b: u8) -> fmt::Result {
        let c = b as char;
        if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
            self.wtr.write_char(c)
        } else {
            write!(self.wtr, "(?-u:\\x{:02X})", b)
        }
    }
    fn write_literal_class_byte(&mut self, b: u8) -> fmt::Result {
        let c = b as char;
        if c <= 0x7F as char && !c.is_control() && !c.is_whitespace() {
            self.wtr.write_char(c)
        } else {
            write!(self.wtr, "\\x{:02X}", b)
        }
    }
}
impl Hir {
    pub fn kind(&self) -> &HirKind {
        &self.kind
    }
    pub fn into_kind(mut self) -> HirKind {}
    pub fn empty() -> Hir {}
    pub fn literal(lit: Literal) -> Hir {}
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {}
    pub fn group(group: Group) -> Hir {}
    pub fn concat(mut exprs: Vec<Hir>) -> Hir {}
    pub fn alternation(mut exprs: Vec<Hir>) -> Hir {}
    pub fn dot(bytes: bool) -> Hir {}
    pub fn any(bytes: bool) -> Hir {}
    pub fn is_always_utf8(&self) -> bool {}
    pub fn is_all_assertions(&self) -> bool {}
    pub fn is_anchored_start(&self) -> bool {}
    pub fn is_anchored_end(&self) -> bool {}
    pub fn is_any_anchored_start(&self) -> bool {}
    pub fn is_any_anchored_end(&self) -> bool {}
    pub fn is_match_empty(&self) -> bool {}
}
impl ClassUnicodeRange {
    pub fn new(start: char, end: char) -> ClassUnicodeRange {}
    pub fn start(&self) -> char {
        self.start
    }
    pub fn end(&self) -> char {
        self.end
    }
}
impl ClassUnicode {
    pub fn new<I>(ranges: I) -> ClassUnicode
    where
        I: IntoIterator<Item = ClassUnicodeRange>,
    {}
    pub fn empty() -> ClassUnicode {}
    pub fn push(&mut self, range: ClassUnicodeRange) {}
    pub fn iter(&self) -> ClassUnicodeIter {
        ClassUnicodeIter(self.set.iter())
    }
    pub fn ranges(&self) -> &[ClassUnicodeRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassUnicode) {}
    pub fn intersect(&mut self, other: &ClassUnicode) {}
    pub fn difference(&mut self, other: &ClassUnicode) {}
    pub fn symmetric_difference(&mut self, other: &ClassUnicode) {}
}
impl ClassBytes {
    pub fn new<I>(ranges: I) -> ClassBytes
    where
        I: IntoIterator<Item = ClassBytesRange>,
    {}
    pub fn empty() -> ClassBytes {}
    pub fn push(&mut self, range: ClassBytesRange) {}
    pub fn iter(&self) -> ClassBytesIter {
        ClassBytesIter(self.set.iter())
    }
    pub fn ranges(&self) -> &[ClassBytesRange] {}
    pub fn case_fold_simple(&mut self) {}
    pub fn negate(&mut self) {}
    pub fn union(&mut self, other: &ClassBytes) {}
    pub fn intersect(&mut self, other: &ClassBytes) {}
    pub fn difference(&mut self, other: &ClassBytes) {}
    pub fn symmetric_difference(&mut self, other: &ClassBytes) {}
    pub fn is_all_ascii(&self) -> bool {}
}
