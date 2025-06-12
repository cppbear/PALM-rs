pub fn sort_unstable_keys(&mut self)
    where
        K: Ord,
    {
        self.with_entries(move |entries| {
            entries.sort_unstable_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }