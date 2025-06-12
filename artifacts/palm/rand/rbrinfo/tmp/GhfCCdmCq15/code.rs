fn append_string<R: crate::Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        // Get the max char length to minimize extra space.
        // Limit this check to avoid searching for long slice.
        let max_char_len = if self.slice.len() < 200 {
            self.slice
                .iter()
                .try_fold(1, |max_len, char| {
                    // When the current max_len is 4, the result max_char_len will be 4.
                    Some(max_len.max(char.len_utf8())).filter(|len| *len < 4)
                })
                .unwrap_or(4)
        } else {
            4
        };

        // Split the extension of string to reuse the unused capacities.
        // Skip the split for small length or only ascii slice.
        let mut extend_len = if max_char_len == 1 || len < 100 {
            len
        } else {
            len / 4
        };
        let mut remain_len = len;
        while extend_len > 0 {
            string.reserve(max_char_len * extend_len);
            string.extend(self.sample_iter(&mut *rng).take(extend_len));
            remain_len -= extend_len;
            extend_len = extend_len.min(remain_len);
        }
    }