pub fn into_key_value_mut(self) -> (&'a mut K, &'a mut V) {
        let index = self.index();
        self.entries[index].muts()
    }