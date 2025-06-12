// Answer 0

#[test]
fn test_from_str_valid() {
    struct StrRef<'a>(&'a str);

    let input = StrRef("8080");
    let result = Port::from_str(input);

    assert!(result.is_ok());
    let port = result.unwrap();
    assert_eq!(port.port, 8080);
    assert_eq!(port.repr.as_ref(), "8080");
}

#[test]
fn test_from_str_invalid_non_numeric() {
    struct StrRef<'a>(&'a str);

    let input = StrRef("not_a_port");
    let result = Port::from_str(input);

    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_out_of_bounds() {
    struct StrRef<'a>(&'a str);

    let input = StrRef("70000");
    let result = Port::from_str(input);

    assert!(result.is_err());
}

#[test]
fn test_from_str_low_boundary() {
    struct StrRef<'a>(&'a str);

    let input = StrRef("0");
    let result = Port::from_str(input);

    assert!(result.is_err());
}

#[test]
fn test_from_str_high_boundary() {
    struct StrRef<'a>(&'a str);

    let input = StrRef("65536");
    let result = Port::from_str(input);

    assert!(result.is_err());
}

