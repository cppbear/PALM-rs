pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.0.get())
    }