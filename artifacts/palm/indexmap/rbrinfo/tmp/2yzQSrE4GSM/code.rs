pub fn swap_take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.swap_remove_entry(value).map(|(x, ())| x)
    }