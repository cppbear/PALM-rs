pub const fn new_in(alloc: A) -> Self {
        Self {
            raw: RawTable::new_in(alloc),
        }
    }