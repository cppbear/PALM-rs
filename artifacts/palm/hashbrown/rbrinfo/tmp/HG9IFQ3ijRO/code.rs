fn next(&mut self) -> Option<K> {
        self.inner.next().map(|(k, _)| k)
    }