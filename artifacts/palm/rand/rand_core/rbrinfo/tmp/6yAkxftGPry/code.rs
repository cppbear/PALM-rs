fn seed_from_u64(seed: u64) -> Self {
        Self::new(R::seed_from_u64(seed))
    }