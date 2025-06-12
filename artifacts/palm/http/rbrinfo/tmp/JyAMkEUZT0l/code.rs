pub fn from_str(src: &str) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::try_from_generic(src, |s| Bytes::copy_from_slice(s.as_bytes()))
    }