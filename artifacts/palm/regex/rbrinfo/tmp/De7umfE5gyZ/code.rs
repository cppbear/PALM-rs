pub fn find_iter<'r, 't>(&'r self, text: &'t [u8]) -> Matches<'r, 't> {
        Matches(self.0.searcher().find_iter(text))
    }