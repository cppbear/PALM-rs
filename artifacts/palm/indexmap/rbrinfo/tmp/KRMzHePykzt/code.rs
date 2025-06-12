pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        if let Some(i) = self.get_index_of(key) {
            let entry = &self.as_entries()[i];
            Some(&entry.value)
        } else {
            None
        }
    }