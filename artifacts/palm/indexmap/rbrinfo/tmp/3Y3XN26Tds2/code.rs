pub fn sort_by_cached_key<T, F>(&mut self, mut sort_key: F)
    where
        T: Ord,
        F: FnMut(&K, &V) -> T,
    {
        self.with_entries(move |entries| {
            entries.sort_by_cached_key(move |a| sort_key(&a.key, &a.value));
        });
    }