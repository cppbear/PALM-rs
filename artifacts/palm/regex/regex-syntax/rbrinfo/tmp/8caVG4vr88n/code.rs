fn from(err: hir::Error) -> Error {
        Error::Translate(err)
    }