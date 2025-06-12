fn from_seed(seed: Self::Seed) -> Self {
        // This is for compatibility with 32-bit platforms where Rng::Seed has a different seed size
        // With MSRV >= 1.77: let seed = *seed.first_chunk().unwrap()
        const LEN: usize = core::mem::size_of::<<Rng as SeedableRng>::Seed>();
        let seed = (&seed[..LEN]).try_into().unwrap();
        SmallRng(Rng::from_seed(seed))
    }