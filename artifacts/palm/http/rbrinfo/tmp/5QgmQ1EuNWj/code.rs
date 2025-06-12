fn try_from(s: &String) -> Result<Self, Self::Error> {
        TryFrom::try_from(s.as_bytes())
    }