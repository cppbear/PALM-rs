fn is_word(&self) -> bool {
        self.0 & 0b000000_1_0 > 0
    }