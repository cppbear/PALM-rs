fn from(src: ErrorKind) -> InvalidUriParts {
        InvalidUriParts(src.into())
    }