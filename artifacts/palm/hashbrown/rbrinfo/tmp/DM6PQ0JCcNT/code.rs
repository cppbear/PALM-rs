const fn is_special(self) -> bool {
        self.0 & 0x80 != 0
    }