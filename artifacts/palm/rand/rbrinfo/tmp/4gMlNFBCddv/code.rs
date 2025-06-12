fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // SAFETY: With this distribution we guarantee that we're working with valid ASCII
        // characters.
        // See [#1590](https://github.com/rust-random/rand/issues/1590).
        unsafe {
            let v = string.as_mut_vec();
            v.reserve_exact(len);
            v.extend(self.sample_iter(rng).take(len));
        }
    }