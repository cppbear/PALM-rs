pub unsafe fn insert_unique_unchecked(&mut self, value: T) -> &T {
        self.map.insert_unique_unchecked(value, ()).0
    }