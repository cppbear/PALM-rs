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
    Str(String),
    ByteBuf(Vec<u8>),
    Bytes(Vec<u8>),
    None,
    Some(Box<Content>),
    Unit,
    Newtype(Box<Content>),
    Seq(Vec<Content>),
    Map(std::collections::HashMap<String, Content>),
}

#[derive(Debug)]
enum Unexpected {
    Bool(bool),
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Char(char),
    Str(String),
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
            Content::String(ref s) => Unexpected::Str(s.clone()),
            Content::Str(ref s) => Unexpected::Str(s.clone()),
            Content::ByteBuf(ref b) => Unexpected::Bytes(b.clone()),
            Content::Bytes(ref b) => Unexpected::Bytes(b.clone()),
            Content::None | Content::Some(_) => Unexpected::Option,
            Content::Unit => Unexpected::Unit,
            Content::Newtype(_) => Unexpected::NewtypeStruct,
            Content::Seq(_) => Unexpected::Seq,
            Content::Map(_) => Unexpected::Map,
        }
    }
}

#[test]
fn test_unexpected_with_seq_content() {
    let seq_content = Content::Seq(vec![Content::U8(1), Content::U16(2), Content::Char('a')]);
    let unexpected = seq_content.unexpected();
    match unexpected {
        Unexpected::Seq => assert!(true),
        _ => assert!(false, "Expected Unexpected::Seq"),
    }
}

