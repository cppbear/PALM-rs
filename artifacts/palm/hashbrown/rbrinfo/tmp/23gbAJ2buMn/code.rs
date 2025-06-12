fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(IterHash {
                inner: self.inner.clone(),
                marker: PhantomData,
            })
            .finish()
    }