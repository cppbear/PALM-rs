pub fn insert(self, value: T) -> &'a mut T {
        self.try_insert(value).expect("size overflows MAX_SIZE")
    }