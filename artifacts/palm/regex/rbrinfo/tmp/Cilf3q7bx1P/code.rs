pub fn is_match(&self, text: &str) -> bool {
        self.is_match_at(text, 0)
    }