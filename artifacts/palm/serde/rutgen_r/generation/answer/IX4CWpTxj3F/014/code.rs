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
    Other(String),
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
            Other(ref other) => formatter.write_str(other),
        }
    }
}

#[test]
fn test_fmt_char_variant() {
    let unexpected_char = Unexpected::Char('a');
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected_char.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_char_variant_empty() {
    let unexpected_char = Unexpected::Char(' ');
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected_char.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn test_fmt_char_variant_special() {
    let unexpected_char = Unexpected::Char('\n');
    let mut formatter = std::fmt::Formatter::new();
    let result = unexpected_char.fmt(&mut formatter);
    assert_eq!(result.is_ok(), true);
}

