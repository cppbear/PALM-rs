fn child(&self) -> ClassInduct<'a> {
        match *self {
            ClassFrame::Union { head, .. } => ClassInduct::Item(head),
            ClassFrame::Binary { op, .. } => ClassInduct::BinaryOp(op),
            ClassFrame::BinaryLHS { ref lhs, .. } => {
                ClassInduct::from_set(lhs)
            }
            ClassFrame::BinaryRHS { ref rhs, .. } => {
                ClassInduct::from_set(rhs)
            }
        }
    }