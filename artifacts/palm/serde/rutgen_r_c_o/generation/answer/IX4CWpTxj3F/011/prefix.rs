// Answer 0

#[test]
fn test_fmt_unit() {
    let unexp = Unexpected::Unit;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_bool() {
    let unexp = Unexpected::Bool(true);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsigned() {
    let unexp = Unexpected::Unsigned(42);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_signed() {
    let unexp = Unexpected::Signed(-42);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_float() {
    let unexp = Unexpected::Float(3.14);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_char() {
    let unexp = Unexpected::Char('a');
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_str() {
    let unexp = Unexpected::Str("example");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_bytes() {
    let unexp = Unexpected::Bytes(&[1, 2, 3]);
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_option() {
    let unexp = Unexpected::Option;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_newtype_struct() {
    let unexp = Unexpected::NewtypeStruct;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_seq() {
    let unexp = Unexpected::Seq;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_map() {
    let unexp = Unexpected::Map;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_enum() {
    let unexp = Unexpected::Enum;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_unit_variant() {
    let unexp = Unexpected::UnitVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_newtype_variant() {
    let unexp = Unexpected::NewtypeVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_tuple_variant() {
    let unexp = Unexpected::TupleVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_struct_variant() {
    let unexp = Unexpected::StructVariant;
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

#[test]
fn test_fmt_other() {
    let unexp = Unexpected::Other("unexpected thing");
    let mut formatter = std::fmt::Formatter::new();
    let _ = unexp.fmt(&mut formatter);
}

