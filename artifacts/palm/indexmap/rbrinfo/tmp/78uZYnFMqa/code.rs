pub fn get_index(&self, index: usize) -> Option<&T> {
        self.entries.get(index).map(Bucket::key_ref)
    }