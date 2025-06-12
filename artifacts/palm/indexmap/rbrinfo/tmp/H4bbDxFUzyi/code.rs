pub fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {
        let index = self.index();
        self.entries[index].muts()
    }