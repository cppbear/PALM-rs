fn pop_class(&self, induct: ClassFrame<'a>) -> Option<ClassFrame<'a>> {
        match induct {
            ClassFrame::Union { tail, .. } => {
                if tail.is_empty() {
                    None
                } else {
                    Some(ClassFrame::Union {
                        head: &tail[0],
                        tail: &tail[1..],
                    })
                }
            }
            ClassFrame::Binary {..} => None,
            ClassFrame::BinaryLHS { op, rhs, .. } => {
                Some(ClassFrame::BinaryRHS {
                    op: op,
                    rhs: rhs,
                })
            }
            ClassFrame::BinaryRHS {..} => None,
        }
    }