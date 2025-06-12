pub fn choice<I>(&mut self, iter: I) -> Option<I::Item>
    where
        I: IntoIterator,
        I::IntoIter: ExactSizeIterator,
    {
        let mut iter = iter.into_iter();

        // Get the item at a random index.
        let len = iter.len();
        if len == 0 {
            return None;
        }
        let index = self.usize(0..len);

        iter.nth(index)
    }