pub fn shortest_match_at(
        &self,
        text: &[u8],
        start: usize,
    ) -> Option<usize> {
        self.0.searcher().shortest_match_at(text, start)
    }