pub fn into_mut(self) -> &'a mut V {
        &mut self.map.entries[self.index].value
    }