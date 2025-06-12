pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.symbols).unwrap()
    }