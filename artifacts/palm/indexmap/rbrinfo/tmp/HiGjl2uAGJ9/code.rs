pub fn from_key<Q>(self, key: &Q) -> RawEntryMut<'a, K, V, S>
    where
        S: BuildHasher,
        Q: ?Sized + Hash + Equivalent<K>,
    {
        let hash = self.map.hash(key);
        self.from_key_hashed_nocheck(hash.get(), key)
    }