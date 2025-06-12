fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        StatusCode::from_bytes(t)
    }