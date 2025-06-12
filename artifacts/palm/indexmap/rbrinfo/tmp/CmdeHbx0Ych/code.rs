pub const fn with_hasher(hash_builder: S) -> Self {
        IndexSet {
            map: IndexMap::with_hasher(hash_builder),
        }
    }