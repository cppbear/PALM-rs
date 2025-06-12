fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }