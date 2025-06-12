pub fn as_str(&self) -> &'t str {
        &self.text[self.start..self.end]
    }