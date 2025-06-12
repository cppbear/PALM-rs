fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        HeaderValue::from_shared(vec.into())
    }