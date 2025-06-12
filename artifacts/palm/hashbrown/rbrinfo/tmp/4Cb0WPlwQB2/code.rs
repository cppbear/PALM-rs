pub(crate) fn invert(self) -> Self {
        BitMask(self.0 ^ BITMASK_MASK)
    }