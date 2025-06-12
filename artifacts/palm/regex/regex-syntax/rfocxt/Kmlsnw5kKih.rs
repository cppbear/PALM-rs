use std::char;
use std::cmp;
use std::error;
use std::fmt;
use std::u8;
use ast::Span;
use hir::interval::{Interval, IntervalSet, IntervalSetIter};
use unicode;
pub use hir::visitor::{Visitor, visit};
macro_rules! define_bool {
    ($bit:expr, $is_fn_name:ident, $set_fn_name:ident) => {
        fn $is_fn_name (& self) -> bool { self.bools & (0b1 << $bit) > 0 } fn
        $set_fn_name (& mut self, yes : bool) { if yes { self.bools |= 1 << $bit; } else
        { self.bools &= ! (1 << $bit); } }
    };
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
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
#[derive(Clone, Debug, Eq, PartialEq)]
struct HirInfo {
    /// Represent yes/no questions by a bitfield to conserve space, since
    /// this is included in every HIR expression.
    ///
    /// If more attributes need to be added, it is OK to increase the size of
    /// this as appropriate.
    bools: u8,
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
impl Hir {
    pub fn kind(&self) -> &HirKind {}
    pub fn into_kind(mut self) -> HirKind {}
    pub fn empty() -> Hir {}
    pub fn literal(lit: Literal) -> Hir {}
    pub fn class(class: Class) -> Hir {}
    pub fn anchor(anchor: Anchor) -> Hir {}
    pub fn word_boundary(word_boundary: WordBoundary) -> Hir {}
    pub fn repetition(rep: Repetition) -> Hir {
        let mut info = HirInfo::new();
        info.set_always_utf8(rep.hir.is_always_utf8());
        info.set_all_assertions(rep.hir.is_all_assertions());
        info.set_anchored_start(!rep.is_match_empty() && rep.hir.is_anchored_start());
        info.set_anchored_end(!rep.is_match_empty() && rep.hir.is_anchored_end());
        info.set_any_anchored_start(rep.hir.is_any_anchored_start());
        info.set_any_anchored_end(rep.hir.is_any_anchored_end());
        info.set_match_empty(rep.is_match_empty() || rep.hir.is_match_empty());
        Hir {
            kind: HirKind::Repetition(rep),
            info: info,
        }
    }
    pub fn group(group: Group) -> Hir {}
    pub fn concat(mut exprs: Vec<Hir>) -> Hir {}
    pub fn alternation(mut exprs: Vec<Hir>) -> Hir {}
    pub fn dot(bytes: bool) -> Hir {}
    pub fn any(bytes: bool) -> Hir {}
    pub fn is_always_utf8(&self) -> bool {
        self.info.is_always_utf8()
    }
    pub fn is_all_assertions(&self) -> bool {
        self.info.is_all_assertions()
    }
    pub fn is_anchored_start(&self) -> bool {
        self.info.is_anchored_start()
    }
    pub fn is_anchored_end(&self) -> bool {
        self.info.is_anchored_end()
    }
    pub fn is_any_anchored_start(&self) -> bool {
        self.info.is_any_anchored_start()
    }
    pub fn is_any_anchored_end(&self) -> bool {
        self.info.is_any_anchored_end()
    }
    pub fn is_match_empty(&self) -> bool {
        self.info.is_match_empty()
    }
}
impl Repetition {
    pub fn is_match_empty(&self) -> bool {
        match self.kind {
            RepetitionKind::ZeroOrOne => true,
            RepetitionKind::ZeroOrMore => true,
            RepetitionKind::OneOrMore => false,
            RepetitionKind::Range(RepetitionRange::Exactly(m)) => m == 0,
            RepetitionKind::Range(RepetitionRange::AtLeast(m)) => m == 0,
            RepetitionKind::Range(RepetitionRange::Bounded(m, _)) => m == 0,
        }
    }
}
impl HirInfo {
    fn new() -> HirInfo {
        HirInfo { bools: 0 }
    }
}
