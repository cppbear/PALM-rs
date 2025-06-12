pub fn shortest_match(&self, text: &str) -> Option<usize> {
        self.shortest_match_at(text, 0)
    }