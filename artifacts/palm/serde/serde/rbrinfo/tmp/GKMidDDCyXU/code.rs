pub fn from_utf8_lossy(bytes: &[u8]) -> Cow<str> {
        String::from_utf8_lossy(bytes)
    }