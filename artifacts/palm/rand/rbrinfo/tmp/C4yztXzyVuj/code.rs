pub fn random_ratio_one_over(&mut self, d: usize) -> bool {
        debug_assert_ne!(d, 0);
        // This uses the same logic as `random_ratio` but is optimized for the case that
        // the starting numerator is one (which it always is for `Sequence::Choose()`)

        // In this case (but not `random_ratio`), this way of calculating c is always accurate
        let c = (usize::BITS - 1 - d.leading_zeros()).min(32);

        if self.flip_c_heads(c) {
            let numerator = 1 << c;
            self.random_ratio(numerator, d)
        } else {
            false
        }
    }