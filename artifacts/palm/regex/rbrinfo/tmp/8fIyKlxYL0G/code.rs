pub fn is_match_at(&self, text: &[u8], start: usize) -> bool {
        self.shortest_match_at(text, start).is_some()
    }