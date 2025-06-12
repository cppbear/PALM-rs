pub fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Self> {
        let range = try_simplify_range(range, self.entries.len())?;
        self.entries.get_mut(range).map(Slice::from_mut_slice)
    }