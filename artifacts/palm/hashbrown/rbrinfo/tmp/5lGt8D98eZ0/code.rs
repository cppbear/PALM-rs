pub fn shrink_to_fit(&mut self) {
        self.table
            .shrink_to(0, make_hasher::<_, V, S>(&self.hash_builder));
    }