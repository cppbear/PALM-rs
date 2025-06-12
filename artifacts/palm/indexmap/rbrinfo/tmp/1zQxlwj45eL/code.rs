pub fn shift_insert(mut self, index: usize, value: V) -> &'a mut V {
        self.map
            .shift_insert_unique(index, self.hash, self.key, value);
        &mut self.map.entries[index].value
    }