fn clone(&self) -> Rng {
        Rng::with_seed(self.0)
    }