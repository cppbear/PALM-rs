pub fn get_key_value(&self) -> (&K, &V) {
        self.entries[self.index()].refs()
    }