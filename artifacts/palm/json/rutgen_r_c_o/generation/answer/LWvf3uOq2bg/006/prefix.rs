// Answer 0

#[test]
fn test_float_key_must_be_finite_nan() {
    let error = ErrorCode::FloatKeyMustBeFinite;
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_float_key_must_be_finite_pos_inf() {
    let error = ErrorCode::FloatKeyMustBeFinite;
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_float_key_must_be_finite_neg_inf() {
    let error = ErrorCode::FloatKeyMustBeFinite;
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

