// Answer 0

#[derive(Debug)]
enum Content {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Char(char),
    String(String),
    Str(&'static str),
    ByteBuf(Vec<u8>),
    Bytes(Vec<u8>),
    None,
    Some(Box<Content>),
    Unit,
    Newtype(Box<Content>),
    Seq(Vec<Content>),
    Map(std::collections::HashMap<Content, Content>),
}

#[derive(Debug, PartialEq)]
enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(&'static str),
    Bytes(Vec<u8>),
    Option,
    Unit,
    NewtypeStruct,
    Seq,
    Map,
}

impl Content {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::Bool(b) => Unexpected::Bool(b),
            Content::U8(n) => Unexpected::Unsigned(n as u64),
            Content::U16(n) => Unexpected::Unsigned(n as u64),
            Content::U32(n) => Unexpected::Unsigned(n as u64),
            Content::U64(n) => Unexpected::Unsigned(n),
            Content::I8(n) => Unexpected::Signed(n as i64),
            Content::I16(n) => Unexpected::Signed(n as i64),
            Content::I32(n) => Unexpected::Signed(n as i64),
            Content::I64(n) => Unexpected::Signed(n),
            Content::F32(f) => Unexpected::Float(f as f64),
            Content::F64(f) => Unexpected::Float(f),
            Content::Char(c) => Unexpected::Char(c),
            Content::String(ref s) => Unexpected::Str(s),
            Content::Str(s) => Unexpected::Str(s),
            Content::ByteBuf(ref b) => Unexpected::Bytes(b.clone()),
            Content::Bytes(b) => Unexpected::Bytes(b.clone()),
            Content::None | Content::Some(_) => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            Content::Newtype(_) => Unexpected::NewtypeStruct,
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_with_i32() {
    let content = Content::I32(42);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(42));
}

#[test]
fn test_unexpected_with_negative_i32() {
    let content = Content::I32(-10);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-10));
} 

#[test]
fn test_unexpected_with_zero_i32() {
    let content = Content::I32(0);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(0));
}

