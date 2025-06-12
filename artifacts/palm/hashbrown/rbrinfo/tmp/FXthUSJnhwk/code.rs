pub const fn with_hasher(hasher: S) -> Self {
        Self {
            map: HashMap::with_hasher(hasher),
        }
    }