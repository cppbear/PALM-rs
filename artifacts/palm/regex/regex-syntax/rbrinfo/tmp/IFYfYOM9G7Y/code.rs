pub fn span(&self) -> &Span {
        match *self {
            ClassSet::Item(ref x) => x.span(),
            ClassSet::BinaryOp(ref x) => &x.span,
        }
    }