pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.entries.get(index).map(Bucket::refs)
    }