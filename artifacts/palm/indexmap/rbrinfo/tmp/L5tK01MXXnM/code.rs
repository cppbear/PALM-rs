pub fn from_key<Q>(self, key: &Q) -> Option<(&'a K, &'a V)>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        self.map.get_key_value(key)
    }