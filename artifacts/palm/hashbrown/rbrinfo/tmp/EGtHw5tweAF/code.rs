pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, hasher),
        }
    }