fn try_from(s: String) -> Result<Self, Self::Error> {
        PathAndQuery::from_shared(s.into())
    }