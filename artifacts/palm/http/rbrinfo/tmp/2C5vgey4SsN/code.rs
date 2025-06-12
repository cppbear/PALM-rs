fn from(uri: Uri) -> Self {
        Self {
            parts: Ok(uri.into_parts()),
        }
    }