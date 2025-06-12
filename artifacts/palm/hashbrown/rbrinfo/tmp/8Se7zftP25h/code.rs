pub fn get<Q>(&self, value: &Q) -> Option<&T>
    where
        Q: Hash + Equivalent<T> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.map.get_key_value(value) {
            Some((k, _)) => Some(k),
            None => None,
        }
    }