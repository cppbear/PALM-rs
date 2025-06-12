pub fn shift_remove<Q>(&mut self, value: &Q) -> bool
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.shift_remove(value).is_some()
    }