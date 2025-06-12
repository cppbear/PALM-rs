pub fn iter_hash(&self, hash: u64) -> IterHash<'_, T> {
        IterHash {
            inner: unsafe { self.raw.iter_hash(hash) },
            marker: PhantomData,
        }
    }