// Answer 0

#[test]
fn test_fmt_neg_int() {
    let number = Number { n: N::NegInt(-42) };
    let mut buffer = String::new();
    let result = number.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "-42");
}

#[test]
fn test_fmt_pos_int() {
    let number = Number { n: N::PosInt(42) };
    let mut buffer = String::new();
    let result = number.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "42");
}

#[test]
fn test_fmt_float() {
    let number = Number { n: N::Float(42.0) };
    let mut buffer = String::new();
    let result = number.fmt(&mut buffer);
    assert!(result.is_ok());
    assert_eq!(buffer, "42");
}

