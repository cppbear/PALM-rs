pub fn key(&self) -> &HeaderName {
        &self.map.entries[self.index].key
    }