pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &Value)>
    where
        String: Borrow<Q>,
        Q: ?Sized + Ord + Eq + Hash,
    {
        self.map.get_key_value(key)
    }