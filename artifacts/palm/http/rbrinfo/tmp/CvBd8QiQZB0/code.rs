fn from_str(s: &str) -> Result<Uri, InvalidUri> {
        Uri::try_from(s.as_bytes())
    }