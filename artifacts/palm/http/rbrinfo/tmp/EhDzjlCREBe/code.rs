fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }