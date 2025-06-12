fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        PathAndQuery::from_shared(Bytes::copy_from_slice(s))
    }