fn default() -> Self {
        IterHashMut {
            inner: Default::default(),
            marker: PhantomData,
        }
    }