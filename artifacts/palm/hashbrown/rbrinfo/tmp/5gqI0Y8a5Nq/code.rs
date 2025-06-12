fn next(&mut self) -> Option<K> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.iter.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }