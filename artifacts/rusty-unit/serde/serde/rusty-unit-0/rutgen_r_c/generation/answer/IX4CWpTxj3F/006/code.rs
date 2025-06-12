// Answer 0

#[test]
fn test_unexpected_enum() {
    use std::fmt;

    struct UnexpectedEnum;

    impl fmt::Display for UnexpectedEnum {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "unexpected enum variant")
        }
    }

    let unexpected_variant = Unexpected::Enum;
    let mut buffer = Vec::new();
    let result = unexpected_variant.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "enum");
}

#[test]
fn test_unexpected_empty_variant() {
    let unexpected_empty = Unexpected::Enum;
    let mut buffer = Vec::new();
    let result = unexpected_empty.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "enum");
}

#[test]
fn test_unexpected_other() {
    let unexpected_other = Unexpected::Other("unrecognized type");
    let mut buffer = Vec::new();
    let result = unexpected_other.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "unrecognized type");
}

#[test]
fn test_unexpected_struct_variant() {
    let unexpected_struct_variant = Unexpected::StructVariant;
    let mut buffer = Vec::new();
    let result = unexpected_struct_variant.fmt(&mut fmt::Formatter::new(&mut buffer));

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "struct variant");
}

