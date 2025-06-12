pub fn sort_by<F>(&mut self, mut cmp: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        self.map.sort_by(move |a, _, b, _| cmp(a, b));
    }