pub fn key_mut(&mut self) -> &mut K {
        let index = self.index();
        &mut self.entries[index].key
    }