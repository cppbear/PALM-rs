pub fn split<'r, 't>(&'r self, text: &'t [u8]) -> Split<'r, 't> {
        Split {
            finder: self.find_iter(text),
            last: 0,
        }
    }