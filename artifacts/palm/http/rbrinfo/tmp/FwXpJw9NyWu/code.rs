fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        Self::from_bytes(s)
    }