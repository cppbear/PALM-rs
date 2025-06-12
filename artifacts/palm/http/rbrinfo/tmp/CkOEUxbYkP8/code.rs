fn from(src: ErrorKind) -> InvalidUri {
        InvalidUri(src)
    }