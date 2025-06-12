fn approximate_size(&self) -> usize {
        self.cache.size + self.prog.approximate_size()
    }