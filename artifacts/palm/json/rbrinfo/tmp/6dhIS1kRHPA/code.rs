fn key_must_be_a_string() -> Error {
    Error::syntax(ErrorCode::KeyMustBeAString, 0, 0)
}