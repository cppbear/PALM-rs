fn induct_class(
        &self,
        ast: &ClassInduct<'a>,
    ) -> Option<ClassFrame<'a>> {
        match *ast {
            ClassInduct::Item(&ast::ClassSetItem::Bracketed(ref x)) => {
                match x.kind {
                    ast::ClassSet::Item(ref item) => {
                        Some(ClassFrame::Union {
                            head: item,
                            tail: &[],
                        })
                    }
                    ast::ClassSet::BinaryOp(ref op) => {
                        Some(ClassFrame::Binary { op: op })
                    }
                }
            }
            ClassInduct::Item(&ast::ClassSetItem::Union(ref x)) => {
                if x.items.is_empty() {
                    None
                } else {
                    Some(ClassFrame::Union {
                        head: &x.items[0],
                        tail: &x.items[1..],
                    })
                }
            }
            ClassInduct::BinaryOp(op) => {
                Some(ClassFrame::BinaryLHS {
                    op: op,
                    lhs: &op.lhs,
                    rhs: &op.rhs,
                })
            }
            _ => None,
        }
    }