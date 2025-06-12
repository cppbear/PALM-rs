pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> Option<&Slice<T>> {
        let entries = self.as_entries();
        let range = try_simplify_range(range, entries.len())?;
        entries.get(range).map(Slice::from_slice)
    }