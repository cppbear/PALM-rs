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
pub fn visit<V: Visitor>(hir: &Hir, visitor: V) -> Result<V::Output, V::Err> {
    HeapVisitor::new().visit(hir, visitor)
}
