fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_bytes(s.as_ref())
    }