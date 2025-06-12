pub fn uses_bytes(&self) -> bool {
        self.is_bytes || self.is_dfa
    }