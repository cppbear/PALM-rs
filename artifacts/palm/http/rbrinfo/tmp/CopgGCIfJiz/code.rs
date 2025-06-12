fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        HeaderValue::from_bytes(t)
    }