fn shortest_match_at(&self, text: &str, start: usize) -> Option<usize> {
        self.0.shortest_match_at(text.as_bytes(), start)
    }