fn from(e: InvalidHeaderName) -> TryEntryError {
            TryEntryError::InvalidHeaderName(e)
        }