pub fn get(&self) -> &V {
        &self.map.entries[self.index].value
    }