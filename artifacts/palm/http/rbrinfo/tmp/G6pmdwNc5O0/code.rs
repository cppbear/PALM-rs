pub fn len(&self) -> usize {
        self.entries.len() + self.extra_values.len()
    }