fn index(&self, index: usize) -> &K {
        &self.iter.as_slice()[index].key
    }