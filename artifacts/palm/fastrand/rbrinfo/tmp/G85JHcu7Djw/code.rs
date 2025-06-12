fn drop(&mut self) {
        self.rng.set(Rng(self.current.0));
    }