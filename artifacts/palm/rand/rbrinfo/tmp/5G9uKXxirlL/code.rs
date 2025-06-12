fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        <[T] as Fill>::fill(self, rng)
    }