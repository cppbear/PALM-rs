fn new(name: &'static str) -> Self {
            TagOrContentVisitor {
                name,
                value: PhantomData,
            }
        }