pub fn port(&self) -> Option<Port<&str>> {
        let bytes = self.as_str();
        bytes
            .rfind(':')
            .and_then(|i| Port::from_str(&bytes[i + 1..]).ok())
    }