// Answer 0

#[derive(Debug)]
enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(&'static str),
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
        use self::Unexpected::*;
        match *self {
            Bool(b) => write!(formatter, "boolean `{}`", b),
            Unsigned(i) => write!(formatter, "integer `{}`", i),
            Signed(i) => write!(formatter, "integer `{}`", i),
            Float(f) => write!(formatter, "floating point `{}`", f),
            Char(c) => write!(formatter, "character `{}`", c),
            Str(s) => write!(formatter, "string {:?}", s),
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
fn test_fmt_with_bool() {
    let value = Unexpected::Bool(true);
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "boolean `true`");
}

#[test]
fn test_fmt_with_unsigned() {
    let value = Unexpected::Unsigned(42);
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "integer `42`");
}

#[test]
fn test_fmt_with_signed() {
    let value = Unexpected::Signed(-42);
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "integer `-42`");
}

#[test]
fn test_fmt_with_float() {
    let value = Unexpected::Float(3.14);
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "floating point `3.14`");
}

#[test]
fn test_fmt_with_char() {
    let value = Unexpected::Char('a');
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "character `a`");
}

#[test]
fn test_fmt_with_str() {
    let value = Unexpected::Str("hello");
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "string \"hello\"");
}

#[test]
fn test_fmt_with_unit() {
    let value = Unexpected::Unit;
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "unit value");
}

#[test]
fn test_fmt_with_option() {
    let value = Unexpected::Option;
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "Option value");
}

#[test]
fn test_fmt_with_other() {
    let value = Unexpected::Other("custom message");
    let mut buffer = std::fmt::Formatter::new();
    let result = value.fmt(&mut buffer);
    assert_eq!(format!("{}", value), "custom message");
}

