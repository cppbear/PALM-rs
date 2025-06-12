pub fn shift_remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        match self.shift_remove_full(key) {
            Some((_, key, value)) => Some((key, value)),
            None => None,
        }
    }