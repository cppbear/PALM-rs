pub fn from_key_hashed_nocheck<Q>(self, hash: u64, key: &Q) -> Option<(&'a K, &'a V)>
    where
        Q: ?Sized + Equivalent<K>,
    {
        let hash = HashValue(hash as usize);
        let i = self.map.core.get_index_of(hash, key)?;
        self.map.get_index(i)
    }