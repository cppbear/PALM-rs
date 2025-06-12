fn size_hint(&self) -> (usize, Option<usize>) {
        let map = self.map;
        debug_assert!(map.entries.len() >= self.entry);

        let lower = map.entries.len() - self.entry;
        // We could pessimistically guess at the upper bound, saying
        // that its lower + map.extra_values.len(). That could be
        // way over though, such as if we're near the end, and have
        // already gone through several extra values...
        (lower, None)
    }