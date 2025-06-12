fn reseed_and_generate(&mut self, results: &mut <Self as BlockRngCore>::Results) {
        trace!("Reseeding RNG (periodic reseed)");

        let num_bytes = size_of_val(results.as_ref());

        if let Err(e) = self.reseed() {
            warn!("Reseeding RNG failed: {}", e);
            let _ = e;
        }

        self.bytes_until_reseed = self.threshold - num_bytes as i64;
        self.inner.generate(results);
    }