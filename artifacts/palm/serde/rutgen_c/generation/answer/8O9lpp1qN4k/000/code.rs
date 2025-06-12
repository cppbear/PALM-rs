// Answer 0

#[test]
fn test_unexpected_bool() {
    let content = Content::Bool(true);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Bool(true));
}

#[test]
fn test_unexpected_u8() {
    let content = Content::U8(255);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(255));
}

#[test]
fn test_unexpected_u16() {
    let content = Content::U16(65535);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(65535));
}

#[test]
fn test_unexpected_u32() {
    let content = Content::U32(4294967295);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(4294967295));
}

#[test]
fn test_unexpected_u64() {
    let content = Content::U64(18446744073709551615);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(18446744073709551615));
}

#[test]
fn test_unexpected_i8() {
    let content = Content::I8(-128);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-128));
}

#[test]
fn test_unexpected_i16() {
    let content = Content::I16(-32768);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-32768));
}

#[test]
fn test_unexpected_i32() {
    let content = Content::I32(-2147483648);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-2147483648));
}

#[test]
fn test_unexpected_i64() {
    let content = Content::I64(-9223372036854775808);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-9223372036854775808));
}

#[test]
fn test_unexpected_f32() {
    let content = Content::F32(1.23);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(1.23));
}

#[test]
fn test_unexpected_f64() {
    let content = Content::F64(2.718281828);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(2.718281828));
}

#[test]
fn test_unexpected_char() {
    let content = Content::Char('c');
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Char('c'));
}

#[test]
fn test_unexpected_string() {
    let content = Content::String(String::from("hello"));
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Str("hello"));
}

#[test]
fn test_unexpected_str() {
    let content = Content::Str("world");
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Str("world"));
}

#[test]
fn test_unexpected_byte_buf() {
    let content = Content::ByteBuf(vec![1, 2, 3]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Bytes(&[1, 2, 3][..]));
}

#[test]
fn test_unexpected_bytes() {
    let content = Content::Bytes(&[4, 5, 6]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Bytes(&[4, 5, 6][..]));
}

#[test]
fn test_unexpected_none() {
    let content = Content::None;
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Option);
}

#[test]
fn test_unexpected_some() {
    let content = Content::Some(Box::new(Content::Bool(true)));
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Option);
}

#[test]
fn test_unexpected_unit() {
    let content = Content::Unit;
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unit);
}

#[test]
fn test_unexpected_newtype() {
    let content = Content::Newtype(Box::new(Content::Bool(true)));
    let result = content.unexpected();
    assert_eq!(result, Unexpected::NewtypeStruct);
}

#[test]
fn test_unexpected_seq() {
    let content = Content::Seq(vec![Content::Bool(true)]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Seq);
}

#[test]
fn test_unexpected_map() {
    let content = Content::Map(vec![(Content::Str("key"), Content::Str("value"))]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Map);
}

