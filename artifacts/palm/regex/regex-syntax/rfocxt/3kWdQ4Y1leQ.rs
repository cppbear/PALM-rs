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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
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
pub struct Repetition {
    /// The kind of this repetition operator.
    pub kind: RepetitionKind,
    /// Whether this repetition operator is greedy or not. A greedy operator
    /// will match as much as it can. A non-greedy operator will match as
    /// little as it can.
    ///
    /// Typically, operators are greedy by default and are only non-greedy when
    /// a `?` suffix is used, e.g., `(expr)*` is greedy while `(expr)*?` is
    /// not. However, this can be inverted via the `U` "ungreedy" flag.
    pub greedy: bool,
    /// The expression being repeated.
    pub hir: Box<Hir>,
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
}
#[derive(Debug)]
pub struct Printer {
    _priv: (),
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionRange {
    /// Matches a sub-expression exactly this many times.
    Exactly(u32),
    /// Matches a sub-expression at least this many times.
    AtLeast(u32),
    /// Matches a sub-expression at least `m` times and at most `n` times.
    Bounded(u32, u32),
}
#[derive(Debug)]
pub enum Error {
    PropertyNotFound,
    PropertyValueNotFound,
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
impl<'p, W: fmt::Write> Visitor for Writer<'p, W> {
    type Output = ();
    type Err = fmt::Error;
    fn finish(self) -> fmt::Result {}
    fn visit_pre(&mut self, hir: &Hir) -> fmt::Result {}
    fn visit_post(&mut self, hir: &Hir) -> fmt::Result {
        match *hir.kind() {
            HirKind::Empty
            | HirKind::Literal(_)
            | HirKind::Class(_)
            | HirKind::Anchor(_)
            | HirKind::WordBoundary(_)
            | HirKind::Concat(_)
            | HirKind::Alternation(_) => {}
            HirKind::Repetition(ref x) => {
                match x.kind {
                    hir::RepetitionKind::ZeroOrOne => {
                        self.wtr.write_str("?")?;
                    }
                    hir::RepetitionKind::ZeroOrMore => {
                        self.wtr.write_str("*")?;
                    }
                    hir::RepetitionKind::OneOrMore => {
                        self.wtr.write_str("+")?;
                    }
                    hir::RepetitionKind::Range(ref x) => {
                        match *x {
                            hir::RepetitionRange::Exactly(m) => {
                                write!(self.wtr, "{{{}}}", m)?;
                            }
                            hir::RepetitionRange::AtLeast(m) => {
                                write!(self.wtr, "{{{},}}", m)?;
                            }
                            hir::RepetitionRange::Bounded(m, n) => {
                                write!(self.wtr, "{{{},{}}}", m, n)?;
                            }
                        }
                    }
                }
                if !x.greedy {
                    self.wtr.write_str("?")?;
                }
            }
            HirKind::Group(_) => {
                self.wtr.write_str(")")?;
            }
        }
        Ok(())
    }
    fn visit_alternation_in(&mut self) -> fmt::Result {}
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
