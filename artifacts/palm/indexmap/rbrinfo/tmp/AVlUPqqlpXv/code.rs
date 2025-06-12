pub fn key(&self) -> &K {
        &self.entries[self.index()].key
    }