// Answer 0

fn float_key_must_be_finite_test() {
    let error = float_key_must_be_finite();
    assert_eq!(error.code, ErrorCode::FloatKeyMustBeFinite);
    assert_eq!(error.line, 0);
    assert_eq!(error.column, 0);
}

