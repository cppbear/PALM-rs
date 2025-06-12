fn from(e: MaxSizeReached) -> TryEntryError {
            TryEntryError::MaxSizeReached(e)
        }