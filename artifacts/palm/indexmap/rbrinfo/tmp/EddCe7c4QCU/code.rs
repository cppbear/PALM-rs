pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self.as_entries())
    }