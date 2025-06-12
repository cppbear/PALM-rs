pub fn complete(&self) -> bool {
        self.complete && !self.is_empty()
    }