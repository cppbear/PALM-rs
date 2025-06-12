pub fn insert_entry(self, value: T) -> OccupiedEntry<'a, T> {
        self.try_insert_entry(value)
            .expect("size overflows MAX_SIZE")
    }