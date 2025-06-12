pub fn read_captures_at<'t>(
        &self,
        locs: &mut Locations,
        text: &'t str,
        start: usize,
    ) -> Option<Match<'t>> {
        self.0
            .searcher_str()
            .read_captures_at(locs, text, start)
            .map(|(s, e)| Match::new(text, s, e))
    }