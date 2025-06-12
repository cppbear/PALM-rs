fn from_seed(seed: Self::Seed) -> Self {
        StdRng(Rng::from_seed(seed))
    }