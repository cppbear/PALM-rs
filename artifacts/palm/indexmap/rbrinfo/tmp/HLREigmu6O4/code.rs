pub fn replace_full(&mut self, value: T) -> (usize, Option<T>) {
        let hash = self.map.hash(&value);
        match self.map.core.replace_full(hash, value, ()) {
            (i, Some((replaced, ()))) => (i, Some(replaced)),
            (i, None) => (i, None),
        }
    }