pub fn take<Q>(&mut self, value: &Q) -> Option<T>
    where
        Q: Hash + Equivalent<T> + ?Sized,
    {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.map.remove_entry(value) {
            Some((k, _)) => Some(k),
            None => None,
        }
    }