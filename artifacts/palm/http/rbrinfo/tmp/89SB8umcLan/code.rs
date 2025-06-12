fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        Authority::from_shared(vec.into())
    }