// Answer 0

#[test]
fn test_fmt_with_bool() {
    let unexpected = Unexpected::Bool(true);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "boolean `true`");
}

#[test]
fn test_fmt_with_unsigned() {
    let unexpected = Unexpected::Unsigned(42);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "integer `42`");
}

#[test]
fn test_fmt_with_signed() {
    let unexpected = Unexpected::Signed(-42);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "integer `-42`");
}

#[test]
fn test_fmt_with_float() {
    let unexpected = Unexpected::Float(3.14);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "floating point `3.14`");
}

#[test]
fn test_fmt_with_char() {
    let unexpected = Unexpected::Char('a');
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "character `a`");
}

#[test]
fn test_fmt_with_str() {
    let unexpected = Unexpected::Str("hello");
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "string \"hello\"");
}

#[test]
fn test_fmt_with_bytes() {
    let unexpected = Unexpected::Bytes(&[1, 2, 3]);
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "byte array");
}

#[test]
fn test_fmt_with_unit() {
    let unexpected = Unexpected::Unit;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "unit value");
}

#[test]
fn test_fmt_with_option() {
    let unexpected = Unexpected::Option;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "Option value");
}

#[test]
fn test_fmt_with_newtype_struct() {
    let unexpected = Unexpected::NewtypeStruct;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "newtype struct");
}

#[test]
fn test_fmt_with_seq() {
    let unexpected = Unexpected::Seq;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "sequence");
}

#[test]
fn test_fmt_with_map() {
    let unexpected = Unexpected::Map;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "map");
}

#[test]
fn test_fmt_with_enum() {
    let unexpected = Unexpected::Enum;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "enum");
}

#[test]
fn test_fmt_with_unit_variant() {
    let unexpected = Unexpected::UnitVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "unit variant");
}

#[test]
fn test_fmt_with_newtype_variant() {
    let unexpected = Unexpected::NewtypeVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "newtype variant");
}

#[test]
fn test_fmt_with_tuple_variant() {
    let unexpected = Unexpected::TupleVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "tuple variant");
}

#[test]
fn test_fmt_with_struct_variant() {
    let unexpected = Unexpected::StructVariant;
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "struct variant");
}

#[test]
fn test_fmt_with_other() {
    let unexpected = Unexpected::Other("unrecognized input");
    let mut output = String::new();
    let result = write!(&mut output, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(output, "unrecognized input");
}

