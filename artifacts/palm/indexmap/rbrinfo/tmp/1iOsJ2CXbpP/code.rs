pub fn swap_remove_full<Q>(&mut self, key: &Q) -> Option<(usize, K, V)>
    where
        Q: ?Sized + Hash + Equivalent<K>,
    {
        match self.as_entries() {
            [x] if key.equivalent(&x.key) => {
                let (k, v) = self.core.pop()?;
                Some((0, k, v))
            }
            [_] | [] => None,
            _ => {
                let hash = self.hash(key);
                self.core.swap_remove_full(hash, key)
            }
        }
    }