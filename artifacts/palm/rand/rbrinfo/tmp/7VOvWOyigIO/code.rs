fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> Result<T, Error> {
        T::Sampler::sample_single(self.start, self.end, rng)
    }