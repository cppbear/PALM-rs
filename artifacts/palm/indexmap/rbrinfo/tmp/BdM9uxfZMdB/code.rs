pub fn with_capacity(n: usize) -> Self {
        IndexSet {
            map: IndexMap::with_capacity(n),
        }
    }