pub fn insert(self, value: V) -> &'a mut V {
        self.insert_entry(value).into_mut()
    }