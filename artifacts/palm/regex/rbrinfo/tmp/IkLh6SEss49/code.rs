pub fn read_captures_at<'t>(
        &self,
        locs: &mut Locations,
        text: &'t [u8],
        start: usize,
    ) -> Option<Match<'t>> {
        self.0.searcher().read_captures_at(locs, text, start)
            .map(|(s, e)| Match::new(text, s, e))
    }