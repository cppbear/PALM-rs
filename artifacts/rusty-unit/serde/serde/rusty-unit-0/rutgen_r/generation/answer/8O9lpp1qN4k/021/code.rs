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
    Bytes(&'static [u8]),
    None,
    Some(Box<Content>),
    Unit,
    Newtype(Box<Content>),
    Seq(Vec<Content>),
    Map(std::collections::HashMap<Content, Content>),
}

#[derive(Debug)]
enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(&'static str),
    Bytes(&'static [u8]),
    Option,
    Unit,
    NewtypeStruct,
    Seq,
    Map,
}

impl Content {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Content::U8(n) => Unexpected::Unsigned(n as u64),
            // other match cases...
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_unexpected_content_u8() {
    let content = Content::U8(42);
    let result = content.unexpected();
    match result {
        Unexpected::Unsigned(n) => assert_eq!(n, 42u64),
        _ => panic!("Expected Unexpected::Unsigned with value 42."),
    }
}

#[test]
fn test_unexpected_content_u8_zero() {
    let content = Content::U8(0);
    let result = content.unexpected();
    match result {
        Unexpected::Unsigned(n) => assert_eq!(n, 0u64),
        _ => panic!("Expected Unexpected::Unsigned with value 0."),
    }
}

#[test]
fn test_unexpected_content_u8_max() {
    let content = Content::U8(u8::MAX);
    let result = content.unexpected();
    match result {
        Unexpected::Unsigned(n) => assert_eq!(n, u8::MAX as u64),
        _ => panic!("Expected Unexpected::Unsigned with value {}", u8::MAX),
    }
}

