pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        self.or_try_insert_with(default)
            .expect("size overflows MAX_SIZE")
    }