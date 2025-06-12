pub fn splice<R, I>(&mut self, range: R, replace_with: I) -> Splice<'_, I::IntoIter, T, S>
    where
        R: RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        Splice::new(self, range, replace_with.into_iter())
    }