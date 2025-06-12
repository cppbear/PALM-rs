pub fn get_range_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Option<&mut Slice<K, V>> {
        let entries = self.as_entries_mut();
        let range = try_simplify_range(range, entries.len())?;
        entries.get_mut(range).map(Slice::from_mut_slice)
    }