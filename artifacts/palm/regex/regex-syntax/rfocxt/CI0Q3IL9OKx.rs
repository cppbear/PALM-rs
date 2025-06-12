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
pub struct Hir {
    /// The underlying HIR kind.
    kind: HirKind,
    /// Analysis info about this HIR, computed during construction.
    info: HirInfo,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionKind {
    /// `?`
    ZeroOrOne,
    /// `*`
    ZeroOrMore,
    /// `+`
    OneOrMore,
    /// `{m,n}`
    Range(RepetitionRange),
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepetitionKind {
    /// Matches a sub-expression zero or one times.
    ZeroOrOne,
    /// Matches a sub-expression zero or more times.
    ZeroOrMore,
    /// Matches a sub-expression one or more times.
    OneOrMore,
    /// Matches a sub-expression within a bounded range of times.
    Range(RepetitionRange),
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
