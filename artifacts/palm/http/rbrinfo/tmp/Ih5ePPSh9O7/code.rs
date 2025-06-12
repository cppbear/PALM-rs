fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        PathAndQuery::from_shared(vec.into())
    }