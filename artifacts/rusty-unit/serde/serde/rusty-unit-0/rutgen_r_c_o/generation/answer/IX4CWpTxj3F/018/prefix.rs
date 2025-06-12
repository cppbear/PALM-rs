// Answer 0

#[test]
fn test_fmt_bool_true() {
    use std::fmt::Formatter;
    let b = true;
    let unexpected = Unexpected::Bool(b);
    let mut formatter = Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

#[test]
fn test_fmt_bool_false() {
    use std::fmt::Formatter;
    let b = false;
    let unexpected = Unexpected::Bool(b);
    let mut formatter = Formatter::new();
    let _ = unexpected.fmt(&mut formatter);
}

