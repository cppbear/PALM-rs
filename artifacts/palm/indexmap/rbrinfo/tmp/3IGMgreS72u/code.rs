pub fn and_modify<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut K, &mut V),
    {
        if let Self::Occupied(entry) = &mut self {
            let (k, v) = entry.get_key_value_mut();
            f(k, v);
        }
        self
    }