fn get_index_mut2(&mut self, index: usize) -> Option<(&mut K, &mut V)> {
        self.as_entries_mut().get_mut(index).map(Bucket::muts)
    }