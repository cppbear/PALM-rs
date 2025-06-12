fn from(err: header::InvalidHeaderName) -> Error {
        Error {
            inner: ErrorKind::HeaderName(err),
        }
    }