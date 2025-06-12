pub fn split<'r, 't>(&'r self, text: &'t str) -> Split<'r, 't> {
        Split {
            finder: self.find_iter(text),
            last: 0,
        }
    }