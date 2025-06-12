pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.0.get())
    }