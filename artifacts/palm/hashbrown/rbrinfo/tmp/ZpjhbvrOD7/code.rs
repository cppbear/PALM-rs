fn default() -> Self {
        Self {
            // SAFETY: Because the table is static, it always outlives the iter.
            inner: unsafe { RawIterHashInner::new(&RawTableInner::NEW, 0) },
            _marker: PhantomData,
        }
    }