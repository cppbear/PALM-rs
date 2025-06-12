pub fn sorted_unstable_by<F>(self, mut cmp: F) -> IntoIter<T>
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut entries = self.into_entries();
        entries.sort_unstable_by(move |a, b| cmp(&a.key, &b.key));
        IntoIter::new(entries)
    }