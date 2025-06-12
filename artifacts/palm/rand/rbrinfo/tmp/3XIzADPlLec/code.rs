fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> X {
        self.0.sample(rng)
    }