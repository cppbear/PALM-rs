pub fn get_mut(&mut self) -> &mut T {
        &mut self.map.entries[self.index].value
    }