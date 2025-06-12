// Answer 0

#[test]
fn test_fmt_bool() {
    let unexpected = Unexpected::Bool(true);
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "boolean `true`");
}

#[test]
fn test_fmt_unsigned() {
    let unexpected = Unexpected::Unsigned(42);
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "integer `42`");
}

#[test]
fn test_fmt_signed() {
    let unexpected = Unexpected::Signed(-42);
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "integer `-42`");
}

#[test]
fn test_fmt_float() {
    let unexpected = Unexpected::Float(3.14);
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "floating point `3.14`");
}

#[test]
fn test_fmt_char() {
    let unexpected = Unexpected::Char('a');
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "character `a`");
}

#[test]
fn test_fmt_str() {
    let unexpected = Unexpected::Str("test");
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "string \"test\"");
}

#[test]
fn test_fmt_bytes() {
    let unexpected = Unexpected::Bytes(&[0, 1, 2]);
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "byte array");
}

#[test]
fn test_fmt_unit() {
    let unexpected = Unexpected::Unit;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "unit value");
}

#[test]
fn test_fmt_option() {
    let unexpected = Unexpected::Option;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "Option value");
}

#[test]
fn test_fmt_newtype_struct() {
    let unexpected = Unexpected::NewtypeStruct;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "newtype struct");
}

#[test]
fn test_fmt_seq() {
    let unexpected = Unexpected::Seq;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "sequence");
}

#[test]
fn test_fmt_map() {
    let unexpected = Unexpected::Map;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "map");
}

#[test]
fn test_fmt_enum() {
    let unexpected = Unexpected::Enum;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "enum");
}

#[test]
fn test_fmt_unit_variant() {
    let unexpected = Unexpected::UnitVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "unit variant");
}

#[test]
fn test_fmt_newtype_variant() {
    let unexpected = Unexpected::NewtypeVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "newtype variant");
}

#[test]
fn test_fmt_tuple_variant() {
    let unexpected = Unexpected::TupleVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "tuple variant");
}

#[test]
fn test_fmt_struct_variant() {
    let unexpected = Unexpected::StructVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "struct variant");
}

#[test]
fn test_fmt_other() {
    let unexpected = Unexpected::Other("unrecognized type");
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
    assert_eq!(formatter.to_string(), "unrecognized type");
}

