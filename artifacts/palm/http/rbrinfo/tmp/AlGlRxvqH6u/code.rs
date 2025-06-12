fn from(err: status::InvalidStatusCode) -> Error {
        Error {
            inner: ErrorKind::StatusCode(err),
        }
    }