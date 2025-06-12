fn from(err: uri::InvalidUri) -> Error {
        Error {
            inner: ErrorKind::Uri(err),
        }
    }