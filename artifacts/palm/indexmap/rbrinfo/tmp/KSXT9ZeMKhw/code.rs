pub fn get_index(&self, index: usize) -> Option<&T> {
        self.as_entries().get(index).map(Bucket::key_ref)
    }