// Answer 0

#[test]
fn test_float_key_must_be_finite_case1() {
    let _result = float_key_must_be_finite();
}

#[test]
fn test_float_key_must_be_finite_case2() {
    let _result = Error::syntax(ErrorCode::FloatKeyMustBeFinite, 1, 0);
}

#[test]
fn test_float_key_must_be_finite_case3() {
    let _result = Error::syntax(ErrorCode::FloatKeyMustBeFinite, 0, 1);
}

#[test]
fn test_float_key_must_be_finite_case4() {
    let _result = Error::syntax(ErrorCode::FloatKeyMustBeFinite, 128, 128);
}

#[test]
fn test_float_key_must_be_finite_case5() {
    let _result = Error::syntax(ErrorCode::FloatKeyMustBeFinite, 64, 64);
}

