fn seed_from_u64(state: u64) -> Self {
        SmallRng(Rng::seed_from_u64(state))
    }