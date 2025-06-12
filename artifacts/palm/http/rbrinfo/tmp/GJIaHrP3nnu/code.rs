fn from_str(t: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(t)
    }