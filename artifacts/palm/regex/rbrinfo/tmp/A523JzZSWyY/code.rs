pub fn find_at<'t>(
        &self,
        text: &'t str,
        start: usize,
    ) -> Option<Match<'t>> {
        self.0.searcher_str().find_at(text, start).map(|(s, e)| {
            Match::new(text, s, e)
        })
    }