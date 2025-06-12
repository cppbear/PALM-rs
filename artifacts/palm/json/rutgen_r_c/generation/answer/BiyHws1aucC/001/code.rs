// Answer 0

#[test]
fn test_fmt_with_pos_int() {
    let number = Number { n: N::PosInt(42) };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let result = number.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(buffer, "Number(42)");
}

#[test]
fn test_fmt_with_neg_int() {
    let number = Number { n: N::NegInt(-42) };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let result = number.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(buffer, "Number(-42)");
}

#[test]
fn test_fmt_with_float() {
    let number = Number { n: N::Float(3.14) };
    let mut buffer = String::new();
    let formatter = &mut fmt::Formatter::new(&mut buffer);
    let result = number.fmt(formatter);
    assert!(result.is_ok());
    assert_eq!(buffer, "Number(3.14)");
}

