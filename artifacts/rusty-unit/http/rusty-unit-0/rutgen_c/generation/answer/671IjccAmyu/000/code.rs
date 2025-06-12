// Answer 0

#[test]
fn test_invalid_header_name_display() {
    struct InvalidHeaderNameTest {
        _priv: (),
    }

    let invalid_header_name = InvalidHeaderNameTest { _priv: () };
    let mut output = String::new();
    let formatter = &mut fmt::Formatter::new(&mut output);
    let result = invalid_header_name.fmt(formatter);

    assert!(result.is_ok());
    assert_eq!(output, "invalid HTTP header name");
}

