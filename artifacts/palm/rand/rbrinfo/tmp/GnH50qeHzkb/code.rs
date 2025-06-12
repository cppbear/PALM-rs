fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> S {
        (self.func)(self.distr.sample(rng))
    }