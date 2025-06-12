fn try_from(t: String) -> Result<Self, Self::Error> {
        Authority::from_shared(t.into())
    }