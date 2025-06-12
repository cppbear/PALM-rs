fn try_from(t: String) -> Result<Self, Self::Error> {
        Uri::from_shared(Bytes::from(t))
    }