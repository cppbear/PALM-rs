fn clone(&self) -> Iter<'a, T> {
        Iter {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }