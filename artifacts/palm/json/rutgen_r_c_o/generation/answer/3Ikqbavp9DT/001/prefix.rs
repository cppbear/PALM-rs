// Answer 0

#[test]
fn test_fmt_positive_float() {
    let number = Number { n: N::Float(1.0) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_large_float() {
    let number = Number { n: N::Float(1.7976931348623157e+308) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_small_float() {
    let number = Number { n: N::Float(2.2250738585072014e-308) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_negative_float() {
    let number = Number { n: N::Float(-1.0) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

#[test]
fn test_fmt_zero_float() {
    let number = Number { n: N::Float(0.0) };
    let mut formatter = fmt::Formatter::new();
    let _ = number.fmt(&mut formatter);
}

