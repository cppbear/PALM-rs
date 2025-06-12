fn is_match(&self) -> bool {
        self.0 & 0b0000000_1 > 0
    }