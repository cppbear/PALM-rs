fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        Uri::from_shared(Bytes::copy_from_slice(t))
    }