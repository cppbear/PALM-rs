pub fn with_capacity_and_hasher_in(capacity: usize, hasher: S, alloc: A) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher_in(capacity, hasher, alloc),
        }
    }