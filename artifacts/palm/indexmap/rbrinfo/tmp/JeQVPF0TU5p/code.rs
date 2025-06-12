pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: ?Sized + Hash + Equivalent<T>,
    {
        self.map.get_key_value(value).map(|(x, &())| x)
    }