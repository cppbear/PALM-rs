fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entries_iter = self.iter.iter().map(|(k, _)| k);
        f.debug_list().entries(entries_iter).finish()
    }