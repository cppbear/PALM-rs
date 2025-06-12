fn clone_from(&mut self, other: &Self) {
        self.hash = other.hash;
        self.key.clone_from(&other.key);
        self.value.clone_from(&other.value);
    }