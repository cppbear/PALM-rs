pub fn find<'t>(&self, text: &'t [u8]) -> Option<Match<'t>> {
        self.find_at(text, 0)
    }