fn from(err: ast::Error) -> Error {
        Error::Parse(err)
    }