fn last(self) -> Option<Self::Item> {
        self.inner.last().map(|b| &b.key)
    }