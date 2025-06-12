fn try_from(t: u16) -> Result<Self, Self::Error> {
        StatusCode::from_u16(t)
    }