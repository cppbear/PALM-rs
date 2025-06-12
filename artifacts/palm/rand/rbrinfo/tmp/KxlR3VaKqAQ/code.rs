fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wrapping<T> {
        Wrapping(rng.random())
    }