fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|b| &b.key)
    }