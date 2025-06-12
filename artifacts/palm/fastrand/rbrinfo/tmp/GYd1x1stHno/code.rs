pub fn fork(&mut self) -> Self {
        Rng::with_seed(self.gen_u64())
    }