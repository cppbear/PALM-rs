fn default() -> Self {
        Self {
            iter: Default::default(),
            allocation: None,
            marker: PhantomData,
        }
    }