// Answer 0

#[test]
fn test_invalid_type_f64_large_negative() {
    let num = ParserNumber::F64(-1e308);
    let exp = &Expected::new("expected floating point number");
    num.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_large_positive() {
    let num = ParserNumber::F64(1e308);
    let exp = &Expected::new("expected floating point number");
    num.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_zero() {
    let num = ParserNumber::F64(0.0);
    let exp = &Expected::new("expected floating point number");
    num.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_small_negative() {
    let num = ParserNumber::F64(-1e-10);
    let exp = &Expected::new("expected floating point number");
    num.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_small_positive() {
    let num = ParserNumber::F64(1e-10);
    let exp = &Expected::new("expected floating point number");
    num.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_exactly_pi() {
    let num = ParserNumber::F64(std::f64::consts::PI);
    let exp = &Expected::new("expected floating point number");
    num.invalid_type(exp);
}

