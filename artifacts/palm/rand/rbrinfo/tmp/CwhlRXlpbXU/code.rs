fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, s: &mut String, len: usize) {
        // A char is encoded with at most four bytes, thus this reservation is
        // guaranteed to be sufficient. We do not shrink_to_fit afterwards so
        // that repeated usage on the same `String` buffer does not reallocate.
        s.reserve(4 * len);
        s.extend(Distribution::<char>::sample_iter(self, rng).take(len));
    }