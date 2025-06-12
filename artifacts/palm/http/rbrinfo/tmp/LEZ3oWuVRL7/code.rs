fn from(err: header::InvalidHeaderValue) -> Error {
        Error {
            inner: ErrorKind::HeaderValue(err),
        }
    }