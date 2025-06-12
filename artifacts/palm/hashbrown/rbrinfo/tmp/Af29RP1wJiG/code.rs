fn search<F>(self, hash: u64, mut is_match: F) -> Option<(&'a K, &'a V)>
    where
        F: FnMut(&K) -> bool,
    {
        match self.map.table.get(hash, |(k, _)| is_match(k)) {
            Some((key, value)) => Some((key, value)),
            None => None,
        }
    }