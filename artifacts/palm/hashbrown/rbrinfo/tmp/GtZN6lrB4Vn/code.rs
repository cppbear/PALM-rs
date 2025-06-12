fn clone(&self) -> IterHash<'a, T> {
        IterHash {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }