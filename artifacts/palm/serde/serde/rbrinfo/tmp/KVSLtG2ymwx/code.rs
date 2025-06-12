fn new(expecting: &'static str) -> Self {
        FromStrVisitor {
            expecting,
            ty: PhantomData,
        }
    }