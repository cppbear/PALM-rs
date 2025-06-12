pub fn sort_keys(&mut self)
    where
        K: Ord,
    {
        self.with_entries(move |entries| {
            entries.sort_by(move |a, b| K::cmp(&a.key, &b.key));
        });
    }