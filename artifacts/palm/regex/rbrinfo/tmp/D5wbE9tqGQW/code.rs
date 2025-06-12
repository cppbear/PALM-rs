pub fn needs_dotstar(&self) -> bool {
        self.is_dfa && !self.is_reverse && !self.is_anchored_start
    }