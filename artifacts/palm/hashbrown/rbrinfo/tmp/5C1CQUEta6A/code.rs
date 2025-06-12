fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
            marker: PhantomData,
        }
    }