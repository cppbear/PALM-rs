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
impl<'a> HeapVisitor<'a> {
    fn new() -> HeapVisitor<'a> {
        HeapVisitor { stack: vec![] }
    }
    fn visit<V: Visitor>(
        &mut self,
        mut hir: &'a Hir,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {}
    fn induct(&mut self, hir: &'a Hir) -> Option<Frame<'a>> {}
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {}
}
