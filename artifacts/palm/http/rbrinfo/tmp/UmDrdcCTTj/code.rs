pub fn is_idempotent(&self) -> bool {
        match self.0 {
            Put | Delete => true,
            _ => self.is_safe(),
        }
    }