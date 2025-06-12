fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }