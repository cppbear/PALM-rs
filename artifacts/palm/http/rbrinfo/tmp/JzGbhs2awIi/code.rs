fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n).map(|b| &b.key)
    }