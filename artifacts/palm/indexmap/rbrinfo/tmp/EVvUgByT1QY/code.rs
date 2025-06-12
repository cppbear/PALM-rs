pub fn get_index_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.as_entries_mut().get_mut(index).map(Bucket::ref_mut)
    }