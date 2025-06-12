pub fn last_mut(&mut self) -> Option<(&K, &mut V)> {
        self.as_entries_mut().last_mut().map(Bucket::ref_mut)
    }