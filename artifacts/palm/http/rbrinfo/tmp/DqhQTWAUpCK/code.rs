fn from(err: MaxSizeReached) -> Error {
        Error {
            inner: ErrorKind::MaxSizeReached(err),
        }
    }