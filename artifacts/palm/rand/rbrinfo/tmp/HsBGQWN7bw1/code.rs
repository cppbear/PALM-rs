fn generate(&mut self, results: &mut Self::Results) {
        if self.bytes_until_reseed <= 0 {
            // We get better performance by not calling only `reseed` here
            // and continuing with the rest of the function, but by directly
            // returning from a non-inlined function.
            return self.reseed_and_generate(results);
        }
        let num_bytes = size_of_val(results.as_ref());
        self.bytes_until_reseed -= num_bytes as i64;
        self.inner.generate(results);
    }