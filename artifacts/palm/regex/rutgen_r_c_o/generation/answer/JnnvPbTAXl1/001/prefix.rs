// Answer 0

#[test]
#[should_panic]
fn test_fmt_property_not_found() {
    let error = Error::PropertyNotFound;
    let mut formatter = std::fmt::Formatter::default();
    error.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_property_value_not_found() {
    let error = Error::PropertyValueNotFound;
    let mut formatter = std::fmt::Formatter::default();
    error.fmt(&mut formatter);
}

