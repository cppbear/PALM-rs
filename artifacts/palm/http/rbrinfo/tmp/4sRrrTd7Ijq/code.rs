fn from_shared(src: Bytes) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::try_from_generic(src, std::convert::identity)
    }