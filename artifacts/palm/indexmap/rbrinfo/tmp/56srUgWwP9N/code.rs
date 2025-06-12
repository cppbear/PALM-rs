pub fn from_hash_full<F>(self, hash: u64, is_match: F) -> Option<(usize, &'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        let map = self.map;
        let i = self.index_from_hash(hash, is_match)?;
        let (key, value) = map.get_index(i)?;
        Some((i, key, value))
    }