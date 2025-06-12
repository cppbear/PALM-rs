fn float_key_must_be_finite() -> Error {
    Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 0)
}