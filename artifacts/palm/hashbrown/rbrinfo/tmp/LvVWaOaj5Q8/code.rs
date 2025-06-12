pub fn from_hash<F>(self, hash: u64, is_match: F) -> RawEntryMut<'a, K, V, S, A>
    where
        for<'b> F: FnMut(&'b K) -> bool,
    {
        self.search(hash, is_match)
    }