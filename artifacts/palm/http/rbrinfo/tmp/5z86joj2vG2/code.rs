fn from(err: uri::InvalidUriParts) -> Error {
        Error {
            inner: ErrorKind::UriParts(err),
        }
    }