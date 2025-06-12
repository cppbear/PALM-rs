pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.map.sort_keys()
    }