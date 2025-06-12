pub fn with_capacity(capacity: usize) -> Self {
        Self {
            raw: RawTable::with_capacity(capacity),
        }
    }