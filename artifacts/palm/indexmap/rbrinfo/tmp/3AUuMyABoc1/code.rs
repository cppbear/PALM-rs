pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.map.sort_unstable_keys()
    }