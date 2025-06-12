pub fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        match self.as_entries() {
            [] => None,
            [x] => key.equivalent(&x.key).then_some(0),
            _ => {
                let hash = self.hash(key);
                self.core.get_index_of(hash, key)
            }
        }
    }