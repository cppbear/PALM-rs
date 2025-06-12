fn read_captures_at(
        &self,
        locs: &mut Locations,
        text: &str,
        start: usize,
    ) -> Option<(usize, usize)> {
        self.0.read_captures_at(locs, text.as_bytes(), start)
    }