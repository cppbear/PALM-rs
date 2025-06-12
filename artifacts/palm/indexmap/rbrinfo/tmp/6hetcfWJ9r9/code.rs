pub fn insert_key(&mut self, key: K) -> K {
        mem::replace(self.key_mut(), key)
    }