pub fn contains(&self, value: usize) -> bool {
        let i = self.sparse[value];
        i < self.size && self.dense[i] == value
    }