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
    fn new() -> HeapVisitor<'a> {}
    fn visit<V: Visitor>(
        &mut self,
        mut hir: &'a Hir,
        mut visitor: V,
    ) -> Result<V::Output, V::Err> {
        self.stack.clear();
        visitor.start();
        loop {
            visitor.visit_pre(hir)?;
            if let Some(x) = self.induct(hir) {
                let child = x.child();
                self.stack.push((hir, x));
                hir = child;
                continue;
            }
            visitor.visit_post(hir)?;
            loop {
                let (post_hir, frame) = match self.stack.pop() {
                    None => return visitor.finish(),
                    Some((post_hir, frame)) => (post_hir, frame),
                };
                if let Some(x) = self.pop(frame) {
                    if let Frame::Alternation { .. } = x {
                        visitor.visit_alternation_in()?;
                    }
                    hir = x.child();
                    self.stack.push((post_hir, x));
                    break;
                }
                visitor.visit_post(post_hir)?;
            }
        }
    }
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
    fn pop(&self, induct: Frame<'a>) -> Option<Frame<'a>> {
        match induct {
            Frame::Repetition(_) => None,
            Frame::Group(_) => None,
            Frame::Concat { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(Frame::Concat {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
            Frame::Alternation { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(Frame::Alternation {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
        }
    }
}
impl<'a> Frame<'a> {
    fn child(&self) -> &'a Hir {
        match *self {
            Frame::Repetition(rep) => &rep.hir,
            Frame::Group(group) => &group.hir,
            Frame::Concat { head, .. } => head,
            Frame::Alternation { head, .. } => head,
        }
    }
}
