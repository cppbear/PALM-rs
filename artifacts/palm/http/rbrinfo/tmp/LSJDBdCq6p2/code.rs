fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        Uri::from_shared(Bytes::from(vec))
    }