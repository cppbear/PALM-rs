pub fn find_at<'t>(
        &self,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {
        self.0.searcher().find_at(text, start)
            .map(|(s, e)| Match::new(text, s, e))
    }