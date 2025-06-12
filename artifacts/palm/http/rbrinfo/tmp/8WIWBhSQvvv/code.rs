fn get2<K>(&self, key: &K) -> Option<&T>
    where
        K: AsHeaderName,
    {
        match key.find(self) {
            Some((_, found)) => {
                let entry = &self.entries[found];
                Some(&entry.value)
            }
            None => None,
        }
    }