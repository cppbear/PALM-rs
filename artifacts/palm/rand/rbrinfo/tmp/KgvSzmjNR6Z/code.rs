fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> [T; N] {
        array::from_fn(|_| rng.random())
    }