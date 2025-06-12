fn is_match_at(&self, text: &str, start: usize) -> bool {
        self.0.is_match_at(text.as_bytes(), start)
    }