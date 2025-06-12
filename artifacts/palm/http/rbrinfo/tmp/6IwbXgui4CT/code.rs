pub fn get_mut<K>(&mut self, key: K) -> Option<&mut T>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((_, found)) => {
                let entry = &mut self.entries[found];
                Some(&mut entry.value)
            }
            None => None,
        }
    }