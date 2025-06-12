pub fn port(&self) -> Option<Port<&str>> {
        self.authority().and_then(|a| a.port())
    }