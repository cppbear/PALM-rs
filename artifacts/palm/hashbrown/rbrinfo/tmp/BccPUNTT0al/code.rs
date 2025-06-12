pub const fn with_hasher_in(hasher: S, alloc: A) -> Self {
        Self {
            map: HashMap::with_hasher_in(hasher, alloc),
        }
    }