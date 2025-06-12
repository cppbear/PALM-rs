fn has_empty(&self) -> bool {
        self.0 & 0b00000_1_00 > 0
    }