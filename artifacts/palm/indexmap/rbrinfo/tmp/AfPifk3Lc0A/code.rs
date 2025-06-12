pub fn index_from_hash<F>(self, hash: u64, mut is_match: F) -> Option<usize>
    where
        F: FnMut(&K) -> bool,
    {
        let hash = HashValue(hash as usize);
        let entries = &*self.map.core.entries;
        let eq = move |&i: &usize| is_match(&entries[i].key);
        self.map.core.indices.find(hash.get(), eq).copied()
    }