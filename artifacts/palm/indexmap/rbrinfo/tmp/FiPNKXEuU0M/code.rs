pub fn sort_by_cached_key<K, F>(&mut self, mut sort_key: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.with_entries(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(&a.key));
        });
    }