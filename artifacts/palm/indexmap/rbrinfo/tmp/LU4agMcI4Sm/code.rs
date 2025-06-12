pub(crate) fn key_mut(&mut self) -> &mut K {
        &mut self.map.entries[self.index].key
    }