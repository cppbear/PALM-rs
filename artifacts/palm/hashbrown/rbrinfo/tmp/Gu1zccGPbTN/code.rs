pub fn reserve(&mut self, additional: usize) {
        self.table
            .reserve(additional, make_hasher::<_, V, S>(&self.hash_builder));
    }