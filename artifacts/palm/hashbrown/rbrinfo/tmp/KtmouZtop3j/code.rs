fn default() -> Self {
        Iter {
            inner: Default::default(),
            marker: PhantomData,
        }
    }