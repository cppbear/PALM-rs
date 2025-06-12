fn try_from(src: &'a Uri) -> Result<Self, Self::Error> {
        Ok(src.clone())
    }