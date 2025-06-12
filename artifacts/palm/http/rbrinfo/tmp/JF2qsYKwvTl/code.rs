fn try_from(src: Parts) -> Result<Self, Self::Error> {
        Uri::from_parts(src)
    }