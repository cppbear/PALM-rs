pub fn or_insert(self, default: T) -> &'a mut T {
        self.or_try_insert(default)
            .expect("size overflows MAX_SIZE")
    }