fn reseed(&mut self) -> Result<(), Rsdr::Error> {
        R::try_from_rng(&mut self.reseeder).map(|result| {
            self.bytes_until_reseed = self.threshold;
            self.inner = result
        })
    }