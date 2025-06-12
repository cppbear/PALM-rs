pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.contains_key(key)
    }