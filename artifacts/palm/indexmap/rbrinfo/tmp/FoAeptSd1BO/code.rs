pub fn get(&self) -> &V {
        &self.entries[self.index()].value
    }