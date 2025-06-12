pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of(key) {
            let entry = &self.as_entries()[i];
            Some((&entry.key, &entry.value))
        } else {
            None
        }
    }