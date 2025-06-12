fn from_str(s: &str) -> Result<HeaderName, InvalidHeaderName> {
        HeaderName::from_bytes(s.as_bytes()).map_err(|_| InvalidHeaderName { _priv: () })
    }