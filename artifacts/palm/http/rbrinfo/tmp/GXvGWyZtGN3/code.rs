pub fn is_success(&self) -> bool {
        (200..300).contains(&self.0.get())
    }