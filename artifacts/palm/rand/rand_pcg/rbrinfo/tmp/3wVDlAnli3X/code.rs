fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);

        // The increment must be odd, hence we discard one bit:
        Lcg64Xsh32::from_state_incr(seed_u64[0], seed_u64[1] | 1)
    }