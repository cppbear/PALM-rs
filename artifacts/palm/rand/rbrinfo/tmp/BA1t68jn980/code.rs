fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u64 {
        rng.next_u64()
    }