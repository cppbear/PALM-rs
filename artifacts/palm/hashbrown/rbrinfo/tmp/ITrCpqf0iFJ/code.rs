pub fn iter_hash_mut(&mut self, hash: u64) -> IterHashMut<'_, T> {
        IterHashMut {
            inner: unsafe { self.raw.iter_hash(hash) },
            marker: PhantomData,
        }
    }