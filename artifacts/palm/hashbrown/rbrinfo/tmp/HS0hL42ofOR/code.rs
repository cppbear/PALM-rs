pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self {
            map: HashMap::with_capacity_in(capacity, alloc),
        }
    }