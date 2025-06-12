use hir::{self, Hir, HirKind};
struct HeapVisitor<'a> {
    /// A stack of `Hir` nodes. This is roughly analogous to the call stack
    /// used in a typical recursive visitor.
    stack: Vec<(&'a Hir, Frame<'a>)>,
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
pub struct Group {
    /// The kind of this group. If it is a capturing group, then the kind
    /// contains the capture group index (and the name, if it is a named
    /// group).
    pub kind: GroupKind,
    /// The expression inside the capturing group, which may be empty.
    pub hir: Box<Hir>,
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
enum Frame<'a> {
    /// A stack frame allocated just before descending into a repetition
    /// operator's child node.
    Repetition(&'a ast::Repetition),
    /// A stack frame allocated just before descending into a group's child
    /// node.
    Group(&'a ast::Group),
    /// The stack frame used while visiting every child node of a concatenation
    /// of expressions.
    Concat {
        /// The child node we are currently visiting.
        head: &'a Ast,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Ast],
    },
    /// The stack frame used while visiting every child node of an alternation
    /// of expressions.
    Alternation {
        /// The child node we are currently visiting.
        head: &'a Ast,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Ast],
    },
}
enum Frame<'a> {
    /// A stack frame allocated just before descending into a repetition
    /// operator's child node.
    Repetition(&'a hir::Repetition),
    /// A stack frame allocated just before descending into a group's child
    /// node.
    Group(&'a hir::Group),
    /// The stack frame used while visiting every child node of a concatenation
    /// of expressions.
    Concat {
        /// The child node we are currently visiting.
        head: &'a Hir,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Hir],
    },
    /// The stack frame used while visiting every child node of an alternation
    /// of expressions.
    Alternation {
        /// The child node we are currently visiting.
        head: &'a Hir,
        /// The remaining child nodes to visit (which may be empty).
        tail: &'a [Hir],
    },
}
impl<'a> HeapVisitor<'a> {
    fn new() -> HeapVisitor<'a> {}
    fn visit<V: Visitor>(
        &mut self,
        mut hir: &'a Hir,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {}
    fn induct(&mut self, hir: &'a Hir) -> Option<Frame<'a>> {
        match *hir.kind() {
            HirKind::Repetition(ref x) => Some(Frame::Repetition(x)),
            HirKind::Group(ref x) => Some(Frame::Group(x)),
            HirKind::Concat(ref x) if x.is_empty() => None,
            HirKind::Concat(ref x) => {
                Some(Frame::Concat {
                    head: &x[0],
                    tail: &x[1..],
                })
            }
            HirKind::Alternation(ref x) if x.is_empty() => None,
            HirKind::Alternation(ref x) => {
                Some(Frame::Alternation {
                    head: &x[0],
                    tail: &x[1..],
                })
            }
            _ => None,
        }
    }
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {}
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
