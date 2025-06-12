fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for (key, value) in self {
            key.hash(state);
            value.hash(state);
        }
    }