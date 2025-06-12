pub fn get(&self) -> &T {
        &self.map.entries[self.index].value
    }