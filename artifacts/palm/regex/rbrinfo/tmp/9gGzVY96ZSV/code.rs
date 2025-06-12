pub fn shortest_match_at(
        &self,
        text: &str,
        start: usize,
    ) -> Option<usize> {
        self.0.searcher_str().shortest_match_at(text, start)
    }