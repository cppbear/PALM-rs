pub fn with_capacity(n: usize) -> Self {
        Self::with_capacity_and_hasher(n, <_>::default())
    }