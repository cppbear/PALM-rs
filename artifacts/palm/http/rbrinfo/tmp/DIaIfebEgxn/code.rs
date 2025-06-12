fn from_str(s: &str) -> Result<Self, InvalidUri> {
        TryFrom::try_from(s)
    }