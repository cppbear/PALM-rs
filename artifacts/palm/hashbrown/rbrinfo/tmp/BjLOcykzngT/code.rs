fn next(&mut self) -> Option<&'a K> {
        // Avoid `Option::map` because it bloats LLVM IR.
        match self.inner.next() {
            Some((k, _)) => Some(k),
            None => None,
        }
    }