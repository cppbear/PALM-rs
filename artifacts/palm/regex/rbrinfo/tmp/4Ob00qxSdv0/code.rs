pub fn is_match(&self, text: &[u8]) -> bool {
        self.is_match_at(text, 0)
    }