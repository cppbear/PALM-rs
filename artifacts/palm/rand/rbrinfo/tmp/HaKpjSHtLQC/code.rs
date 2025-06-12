fn append_string<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        string: &mut alloc::string::String,
        len: usize,
    ) {
        // Getting the hi value to assume the required length to reserve in string.
        let mut hi = self.0.sampler.low + self.0.sampler.range - 1;
        if hi >= CHAR_SURROGATE_START {
            hi += CHAR_SURROGATE_LEN;
        }
        // Get the utf8 length of hi to minimize extra space.
        let max_char_len = char::from_u32(hi).map(char::len_utf8).unwrap_or(4);
        string.reserve(max_char_len * len);
        string.extend(self.sample_iter(rng).take(len))
    }