pub fn into_key(self) -> &'a mut K {
        let index = self.index();
        &mut self.entries[index].key
    }