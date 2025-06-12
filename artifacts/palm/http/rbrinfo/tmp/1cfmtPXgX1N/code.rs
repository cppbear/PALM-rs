pub fn into_mut(self) -> &'a mut T {
        &mut self.map.entries[self.index].value
    }