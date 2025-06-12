pub fn find<'t>(&self, text: &'t str) -> Option<Match<'t>> {
        self.find_at(text, 0)
    }