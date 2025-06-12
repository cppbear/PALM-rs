pub fn retain<F>(&mut self, mut keep: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.map.retain(move |x, &mut ()| keep(x))
    }