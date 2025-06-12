// Answer 0

#[test]
fn test_serialize_unit() {
    use std::fmt;

    let mut formatter = fmt::Formatter::new();
    let result = formatter.serialize_unit();
}

#[test]
fn test_serialize_unit_with_formatter() {
    use std::fmt;

    let mut formatter = fmt::Formatter::new();
    let result = (&mut formatter).serialize_unit();
}

