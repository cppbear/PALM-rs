const fn special_is_empty(self) -> bool {
        debug_assert!(self.is_special());
        self.0 & 0x01 != 0
    }