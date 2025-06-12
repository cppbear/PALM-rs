fn next(&mut self) -> Option<V> {
        self.inner.next().map(|(_, v)| v)
    }