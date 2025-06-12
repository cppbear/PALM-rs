fn clone_from(&mut self, other: &Self) {
        self.core.clone_from(&other.core);
        self.hash_builder.clone_from(&other.hash_builder);
    }