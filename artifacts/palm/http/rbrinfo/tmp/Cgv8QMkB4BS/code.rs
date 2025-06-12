fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s)
    }