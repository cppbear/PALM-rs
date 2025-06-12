pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.as_entries_mut().first_mut().map(Bucket::ref_mut)
    }