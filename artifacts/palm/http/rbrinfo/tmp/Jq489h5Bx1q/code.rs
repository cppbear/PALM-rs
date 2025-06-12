pub fn from_bytes(src: &[u8]) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::try_from_generic(src, Bytes::copy_from_slice)
    }