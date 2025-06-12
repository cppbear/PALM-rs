pub fn first_mut(&mut self) -> Option<(&K, &mut V)> {
        self.entries.first_mut().map(Bucket::ref_mut)
    }