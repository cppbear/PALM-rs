fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index].key
    }