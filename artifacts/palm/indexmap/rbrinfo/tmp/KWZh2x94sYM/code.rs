pub const fn with_hasher(hash_builder: S) -> Self {
        IndexMap {
            core: IndexMapCore::new(),
            hash_builder,
        }
    }