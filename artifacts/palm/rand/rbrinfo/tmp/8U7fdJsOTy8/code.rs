fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        // Make sure to always return true for p = 1.0.
        if self.p_int == ALWAYS_TRUE {
            return true;
        }
        let v: u64 = rng.random();
        v < self.p_int
    }