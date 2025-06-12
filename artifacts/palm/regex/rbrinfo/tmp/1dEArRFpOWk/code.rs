pub fn shortest_match(&self, text: &[u8]) -> Option<usize> {
        self.shortest_match_at(text, 0)
    }