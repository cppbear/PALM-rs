fn default() -> Self {
        Self {
            inner: Default::default(),
            marker: PhantomData,
        }
    }