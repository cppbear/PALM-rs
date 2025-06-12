fn get(&self, key: u32) -> ValueType {
        self.map
            .as_ref()
            .map_or_else(|| Default::default(), |map| map[self.lookup(key)].value)
    }