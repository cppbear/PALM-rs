fn default() -> Self {
        IterHash {
            inner: Default::default(),
            marker: PhantomData,
        }
    }