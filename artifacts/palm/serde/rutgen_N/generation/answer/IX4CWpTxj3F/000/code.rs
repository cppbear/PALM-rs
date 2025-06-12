// Answer 0

#[derive(Debug)]
enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(String),
    Bytes(Vec<u8>),
    Unit,
    Option,
    NewtypeStruct,
    Seq,
    Map,
    Enum,
    UnitVariant,
    NewtypeVariant,
    TupleVariant,
    StructVariant,
    Other(&'static str),
}

impl std::fmt::Display for Unexpected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Unexpected::*;
        match *self {
            Bool(b) => write!(formatter, "boolean `{}`", b),
            Unsigned(i) => write!(formatter, "integer `{}`", i),
            Signed(i) => write!(formatter, "integer `{}`", i),
            Float(f) => write!(formatter, "floating point `{}`", f),
            Char(c) => write!(formatter, "character `{}`", c),
            Str(ref s) => write!(formatter, "string {:?}", s),
            Bytes(_) => formatter.write_str("byte array"),
            Unit => formatter.write_str("unit value"),
            Option => formatter.write_str("Option value"),
            NewtypeStruct => formatter.write_str("newtype struct"),
            Seq => formatter.write_str("sequence"),
            Map => formatter.write_str("map"),
            Enum => formatter.write_str("enum"),
            UnitVariant => formatter.write_str("unit variant"),
            NewtypeVariant => formatter.write_str("newtype variant"),
            TupleVariant => formatter.write_str("tuple variant"),
            StructVariant => formatter.write_str("struct variant"),
            Other(other) => formatter.write_str(other),
        }
    }
}

#[test]
fn test_fmt_bool() {
    let value = Unexpected::Bool(true);
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_unsigned() {
    let value = Unexpected::Unsigned(42);
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_signed() {
    let value = Unexpected::Signed(-42);
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_float() {
    let value = Unexpected::Float(3.14);
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_char() {
    let value = Unexpected::Char('a');
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_str() {
    let value = Unexpected::Str("hello".to_string());
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_bytes() {
    let value = Unexpected::Bytes(vec![1, 2, 3]);
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_unit() {
    let value = Unexpected::Unit;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_option() {
    let value = Unexpected::Option;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_newtype_struct() {
    let value = Unexpected::NewtypeStruct;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_seq() {
    let value = Unexpected::Seq;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_map() {
    let value = Unexpected::Map;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_enum() {
    let value = Unexpected::Enum;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_unit_variant() {
    let value = Unexpected::UnitVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_newtype_variant() {
    let value = Unexpected::NewtypeVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_tuple_variant() {
    let value = Unexpected::TupleVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_struct_variant() {
    let value = Unexpected::StructVariant;
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_other() {
    let value = Unexpected::Other("custom error");
    let mut formatter = std::fmt::Formatter::new();
    let result = value.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

