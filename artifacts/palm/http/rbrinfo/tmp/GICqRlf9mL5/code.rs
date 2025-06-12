pub fn append(&mut self, value: T) {
        let idx = self.index;
        let entry = &mut self.map.entries[idx];
        append_value(idx, entry, &mut self.map.extra_values, value);
    }