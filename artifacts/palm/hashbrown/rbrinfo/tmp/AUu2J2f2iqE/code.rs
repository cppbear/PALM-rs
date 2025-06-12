pub unsafe fn iter_hash(&self, hash: u64) -> RawIterHash<T> {
        RawIterHash::new(self, hash)
    }