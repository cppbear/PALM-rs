// Answer 0

#[test]
fn test_fmt_neg_int_min() {
    let number = Number { n: N::NegInt(-9223372036854775808) };
    let mut buffer = core::fmt::Formatter::new();
    let _ = number.fmt(&mut buffer);
}

#[test]
fn test_fmt_neg_int_mid() {
    let number = Number { n: N::NegInt(-123456789) };
    let mut buffer = core::fmt::Formatter::new();
    let _ = number.fmt(&mut buffer);
}

#[test]
fn test_fmt_neg_int_max() {
    let number = Number { n: N::NegInt(-1) };
    let mut buffer = core::fmt::Formatter::new();
    let _ = number.fmt(&mut buffer);
}

