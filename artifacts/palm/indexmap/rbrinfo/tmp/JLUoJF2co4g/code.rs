pub fn get_full<Q>(&self, value: &Q) -> Option<(usize, &T)>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.get_full(value).map(|(i, x, &())| (i, x))
    }