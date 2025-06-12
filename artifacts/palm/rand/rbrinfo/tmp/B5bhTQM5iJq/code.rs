fn sample_string<R: Rng + ?Sized>(&self, rng: &mut R, len: usize) -> String {
        let mut s = String::new();
        self.append_string(rng, &mut s, len);
        s
    }