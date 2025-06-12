pub(crate) fn leading_zeros(self) -> usize {
        self.0.leading_zeros() as usize / BITMASK_STRIDE
    }