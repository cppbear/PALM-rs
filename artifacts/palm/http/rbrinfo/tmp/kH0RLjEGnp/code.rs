pub fn host(&self) -> Option<&str> {
        self.authority().map(|a| a.host())
    }