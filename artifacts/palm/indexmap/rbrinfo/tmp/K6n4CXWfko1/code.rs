fn reserve_entries(&mut self, additional: usize) {
        reserve_entries(self.entries, additional, self.indices.capacity());
    }