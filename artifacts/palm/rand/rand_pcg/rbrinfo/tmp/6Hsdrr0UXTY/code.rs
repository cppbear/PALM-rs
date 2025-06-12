fn from_seed(seed: Self::Seed) -> Self {
        // Read as if a little-endian u128 value:
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);
        let state = u128::from(seed_u64[0]) | (u128::from(seed_u64[1]) << 64);
        Mcg128Xsl64::new(state)
    }