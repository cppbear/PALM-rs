fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // SAFETY: `self` only samples alphanumeric characters, which are valid UTF-8.
        unsafe {
            let v = string.as_mut_vec();
            v.extend(
                self.sample_iter(rng)
                    .take(len)
                    .inspect(|b| debug_assert!(b.is_ascii_alphanumeric())),
            );
        }
    }