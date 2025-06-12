pub fn is_match_at(&self, text: &str, start: usize) -> bool {
        self.shortest_match_at(text, start).is_some()
    }