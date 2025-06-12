fn try_from(s: &'a String) -> Result<Self, Self::Error> {
        Self::from_bytes(s.as_bytes())
    }