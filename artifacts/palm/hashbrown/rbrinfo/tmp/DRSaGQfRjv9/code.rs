const fn is_full(self) -> bool {
        self.0 & 0x80 == 0
    }