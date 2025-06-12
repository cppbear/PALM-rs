fn next(&mut self) -> Option<T> {
        // Here, self.rng may be a reference, but we must take &mut anyway.
        // Even if sample could take an R: Rng by value, we would need to do this
        // since Rng is not copyable and we cannot enforce that this is "reborrowable".
        Some(self.distr.sample(&mut self.rng))
    }