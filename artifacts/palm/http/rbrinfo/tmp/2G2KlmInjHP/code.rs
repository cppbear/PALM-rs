fn from_str(s: &str) -> Result<HeaderValue, Self::Err> {
        HeaderValue::from_str(s)
    }