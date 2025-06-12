pub fn reserve(&mut self, additional: usize) {
        self.try_reserve(additional)
            .expect("size overflows MAX_SIZE")
    }