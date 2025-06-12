fn flip_c_heads(&mut self, mut c: u32) -> bool {
        debug_assert!(c <= 32);
        // Note that zeros on the left of the chunk represent heads.
        // It needs to be this way round because zeros are filled in when left shifting
        loop {
            let zeros = self.chunk.leading_zeros();

            if zeros < c {
                // The happy path - we found a 1 and can return false
                // Note that because a 1 bit was detected,
                // We cannot have run out of random bits so we don't need to check

                // First consume all of the bits read
                // Using shl seems to give worse performance for size-hinted iterators
                self.chunk = self.chunk.wrapping_shl(zeros + 1);

                self.chunk_remaining = self.chunk_remaining.saturating_sub(zeros + 1);
                return false;
            } else {
                // The number of zeros is larger than `c`
                // There are two possibilities
                if let Some(new_remaining) = self.chunk_remaining.checked_sub(c) {
                    // Those zeroes were all part of our random chunk,
                    // throw away `c` bits of randomness and return true
                    self.chunk_remaining = new_remaining;
                    self.chunk <<= c;
                    return true;
                } else {
                    // Some of those zeroes were part of the random chunk
                    // and some were part of the space behind it
                    // We need to take into account only the zeroes that were random
                    c -= self.chunk_remaining;

                    // Generate a new chunk
                    self.chunk = self.rng.next_u32();
                    self.chunk_remaining = 32;
                    // Go back to start of loop
                }
            }
        }
    }