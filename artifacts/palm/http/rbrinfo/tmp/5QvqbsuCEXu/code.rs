fn try_from(t: String) -> Result<Self, Self::Error> {
        HeaderValue::from_shared(t.into())
    }