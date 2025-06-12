fn remove_lowest_bit(self) -> Self {
        BitMask(self.0 & (self.0 - 1))
    }