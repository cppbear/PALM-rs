pub fn new_in(alloc: A) -> Self {
        Self::with_hasher_in(DefaultHashBuilder::default(), alloc)
    }