pub fn from_hash<F>(self, hash: u64, is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        let map = self.map;
        let i = self.index_from_hash(hash, is_match)?;
        map.get_index(i)
    }