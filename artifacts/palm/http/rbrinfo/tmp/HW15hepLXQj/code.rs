fn from(err: method::InvalidMethod) -> Error {
        Error {
            inner: ErrorKind::Method(err),
        }
    }