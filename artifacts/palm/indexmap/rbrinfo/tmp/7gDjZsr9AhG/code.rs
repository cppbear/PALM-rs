pub fn swap_remove_full<Q>(&mut self, value: &Q) -> Option<(usize, T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.swap_remove_full(value).map(|(i, x, ())| (i, x))
    }