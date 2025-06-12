fn from_seed(seed: Self::Seed) -> Self {
        Self::new(R::from_seed(seed))
    }