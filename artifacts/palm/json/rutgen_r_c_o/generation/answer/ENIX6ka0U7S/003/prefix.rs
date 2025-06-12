// Answer 0

#[test]
fn test_json_unexpected_float_min() {
    let value = -1.7976931348623157E+308;
    let unexpected = JsonUnexpected(de::Unexpected::Float(value));
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

#[test]
fn test_json_unexpected_float_zero() {
    let value = 0.0;
    let unexpected = JsonUnexpected(de::Unexpected::Float(value));
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

#[test]
fn test_json_unexpected_float_max() {
    let value = 1.7976931348623157E+308;
    let unexpected = JsonUnexpected(de::Unexpected::Float(value));
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

#[test]
fn test_json_unexpected_float_small() {
    let value = 1e-10;
    let unexpected = JsonUnexpected(de::Unexpected::Float(value));
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

#[test]
fn test_json_unexpected_float_large() {
    let value = 3.4028234663852886E+38; // maximum for f32
    let unexpected = JsonUnexpected(de::Unexpected::Float(value));
    let mut formatter = fmt::Formatter::new();
    unexpected.fmt(&mut formatter);
}

