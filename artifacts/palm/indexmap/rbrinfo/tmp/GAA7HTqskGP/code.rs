pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.entries[self.index].value
    }