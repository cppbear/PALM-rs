pub fn insert(&mut self, value: usize) {
        let i = self.size;
        self.dense[i] = value;
        self.sparse[value] = i;
        self.size += 1;
    }