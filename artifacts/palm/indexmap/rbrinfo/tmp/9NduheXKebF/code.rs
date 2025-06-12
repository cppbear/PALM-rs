fn retain2<F>(&mut self, mut keep: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        self.map.retain2(move |value, ()| keep(value));
    }