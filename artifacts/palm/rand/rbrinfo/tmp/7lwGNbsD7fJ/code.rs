fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        rng.fill_bytes(self)
    }