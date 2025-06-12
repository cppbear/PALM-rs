// Answer 0

#[test]
fn test_unexpected_bool() {
    let content = Content::Bool(true);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Bool(true));
}

#[test]
fn test_unexpected_u8() {
    let content = Content::U8(150);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(150));
}

#[test]
fn test_unexpected_u16() {
    let content = Content::U16(300);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(300));
}

#[test]
fn test_unexpected_u32() {
    let content = Content::U32(60000);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(60000));
}

#[test]
fn test_unexpected_u64() {
    let content = Content::U64(123456789);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Unsigned(123456789));
}

#[test]
fn test_unexpected_i8() {
    let content = Content::I8(-5);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-5));
}

#[test]
fn test_unexpected_i16() {
    let content = Content::I16(-15);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-15));
}

#[test]
fn test_unexpected_i32() {
    let content = Content::I32(-200);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-200));
}

#[test]
fn test_unexpected_i64() {
    let content = Content::I64(-1000);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Signed(-1000));
}

#[test]
fn test_unexpected_f32() {
    let content = Content::F32(1.5);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(1.5));
}

#[test]
fn test_unexpected_f64() {
    let content = Content::F64(3.141592);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Float(3.141592));
}

#[test]
fn test_unexpected_char() {
    let content = Content::Char('a');
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Char('a'));
}

#[test]
fn test_unexpected_string() {
    let content = Content::String(String::from("test"));
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Str("test"));
}

#[test]
fn test_unexpected_str() {
    let content = Content::Str("hello");
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Str("hello"));
}

#[test]
fn test_unexpected_byte_buf() {
    let content = Content::ByteBuf(vec![1, 2, 3]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Bytes(&[1, 2, 3]));
}

#[test]
fn test_unexpected_bytes() {
    let content = Content::Bytes(&[4, 5, 6]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Bytes(&[4, 5, 6]));
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
    let content = Content::Newtype(Box::new(Content::U8(2)));
    let result = content.unexpected();
    assert_eq!(result, Unexpected::NewtypeStruct);
}

#[test]
fn test_unexpected_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Seq);
}

#[test]
fn test_unexpected_map() {
    let content = Content::Map(vec![(Content::Str("key"), Content::U8(3))]);
    let result = content.unexpected();
    assert_eq!(result, Unexpected::Map);
}

