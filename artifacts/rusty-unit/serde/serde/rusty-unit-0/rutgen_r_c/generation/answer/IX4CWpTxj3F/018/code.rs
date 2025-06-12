// Answer 0

#[test]
fn test_fmt_with_bool_true() {
    let unexpected = Unexpected::Bool(true);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean `true`");
}

#[test]
fn test_fmt_with_bool_false() {
    let unexpected = Unexpected::Bool(false);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "boolean `false`");
}

#[test]
fn test_fmt_with_unsigned() {
    let unexpected = Unexpected::Unsigned(42);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "integer `42`");
}

#[test]
fn test_fmt_with_signed() {
    let unexpected = Unexpected::Signed(-1);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "integer `-1`");
}

#[test]
fn test_fmt_with_float() {
    let unexpected = Unexpected::Float(3.14);
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "floating point `3.14`");
}

#[test]
fn test_fmt_with_char() {
    let unexpected = Unexpected::Char('a');
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "character `a`");
}

#[test]
fn test_fmt_with_str() {
    let unexpected = Unexpected::Str("test");
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "string \"test\"");
}

#[test]
fn test_fmt_with_unit() {
    let unexpected = Unexpected::Unit;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "unit value");
}

#[test]
fn test_fmt_with_option() {
    let unexpected = Unexpected::Option;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "Option value");
}

#[test]
fn test_fmt_with_newtype_struct() {
    let unexpected = Unexpected::NewtypeStruct;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "newtype struct");
}

#[test]
fn test_fmt_with_seq() {
    let unexpected = Unexpected::Seq;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "sequence");
}

#[test]
fn test_fmt_with_map() {
    let unexpected = Unexpected::Map;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "map");
}

#[test]
fn test_fmt_with_enum() {
    let unexpected = Unexpected::Enum;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "enum");
}

#[test]
fn test_fmt_with_unit_variant() {
    let unexpected = Unexpected::UnitVariant;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "unit variant");
}

#[test]
fn test_fmt_with_newtype_variant() {
    let unexpected = Unexpected::NewtypeVariant;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "newtype variant");
}

#[test]
fn test_fmt_with_tuple_variant() {
    let unexpected = Unexpected::TupleVariant;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "tuple variant");
}

#[test]
fn test_fmt_with_struct_variant() {
    let unexpected = Unexpected::StructVariant;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "struct variant");
}

#[test]
fn test_fmt_with_other() {
    let unexpected = Unexpected::Other("unoriginal superhero");
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", unexpected);
    assert!(result.is_ok());
    assert_eq!(buffer, "unoriginal superhero");
}

