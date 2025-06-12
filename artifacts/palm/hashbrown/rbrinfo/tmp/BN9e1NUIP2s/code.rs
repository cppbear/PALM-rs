pub(crate) fn trailing_zeros(self) -> usize {
        // ARM doesn't have a trailing_zeroes instruction, and instead uses
        // reverse_bits (RBIT) + leading_zeroes (CLZ). However older ARM
        // versions (pre-ARMv7) don't have RBIT and need to emulate it
        // instead. Since we only have 1 bit set in each byte on ARM, we can
        // use swap_bytes (REV) + leading_zeroes instead.
        if cfg!(target_arch = "arm") && BITMASK_STRIDE % 8 == 0 {
            self.0.swap_bytes().leading_zeros() as usize / BITMASK_STRIDE
        } else {
            self.0.trailing_zeros() as usize / BITMASK_STRIDE
        }
    }