// Answer 0

#[test]
fn test_unexpected_bool() {
    let content_true = Content::Bool(true);
    let result_true = content_true.unexpected();
    assert_eq!(result_true, Unexpected::Bool(true));

    let content_false = Content::Bool(false);
    let result_false = content_false.unexpected();
    assert_eq!(result_false, Unexpected::Bool(false));
}

#[test]
fn test_unexpected_u8() {
    let content_u8 = Content::U8(255);
    let result = content_u8.unexpected();
    assert_eq!(result, Unexpected::Unsigned(255));
}

#[test]
fn test_unexpected_u16() {
    let content_u16 = Content::U16(65535);
    let result = content_u16.unexpected();
    assert_eq!(result, Unexpected::Unsigned(65535));
}

#[test]
fn test_unexpected_u32() {
    let content_u32 = Content::U32(4294967295);
    let result = content_u32.unexpected();
    assert_eq!(result, Unexpected::Unsigned(4294967295));
}

#[test]
fn test_unexpected_u64() {
    let content_u64 = Content::U64(18446744073709551615);
    let result = content_u64.unexpected();
    assert_eq!(result, Unexpected::Unsigned(18446744073709551615));
}

#[test]
fn test_unexpected_i8() {
    let content_i8 = Content::I8(-128);
    let result = content_i8.unexpected();
    assert_eq!(result, Unexpected::Signed(-128));
}

#[test]
fn test_unexpected_i16() {
    let content_i16 = Content::I16(-32768);
    let result = content_i16.unexpected();
    assert_eq!(result, Unexpected::Signed(-32768));
}

#[test]
fn test_unexpected_i32() {
    let content_i32 = Content::I32(-2147483648);
    let result = content_i32.unexpected();
    assert_eq!(result, Unexpected::Signed(-2147483648));
}

#[test]
fn test_unexpected_i64() {
    let content_i64 = Content::I64(-9223372036854775808);
    let result = content_i64.unexpected();
    assert_eq!(result, Unexpected::Signed(-9223372036854775808));
}

#[test]
fn test_unexpected_f32() {
    let content_f32 = Content::F32(3.14);
    let result = content_f32.unexpected();
    assert_eq!(result, Unexpected::Float(3.14));
}

#[test]
fn test_unexpected_f64() {
    let content_f64 = Content::F64(2.718281828459045);
    let result = content_f64.unexpected();
    assert_eq!(result, Unexpected::Float(2.718281828459045));
}

#[test]
fn test_unexpected_char() {
    let content_char = Content::Char('A');
    let result = content_char.unexpected();
    assert_eq!(result, Unexpected::Char('A'));
}

#[test]
fn test_unexpected_string() {
    let content_string = Content::String(String::from("Hello"));
    let result = content_string.unexpected();
    assert_eq!(result, Unexpected::Str("Hello"));
}

#[test]
fn test_unexpected_str() {
    let content_str = Content::Str("World");
    let result = content_str.unexpected();
    assert_eq!(result, Unexpected::Str("World"));
}

#[test]
fn test_unexpected_bytes() {
    let byte_vec = vec![1, 2, 3];
    let content_bytes = Content::Bytes(byte_vec.clone());
    let result = content_bytes.unexpected();
    assert_eq!(result, Unexpected::Bytes(&byte_vec));
}

#[test]
fn test_unexpected_none() {
    let content_none = Content::None;
    let result = content_none.unexpected();
    assert_eq!(result, Unexpected::Option);
}

#[test]
fn test_unexpected_some() {
    let content_some = Content::Some(Box::new(Content::Bool(true)));
    let result = content_some.unexpected();
    assert_eq!(result, Unexpected::Option);
}

#[test]
fn test_unexpected_unit() {
    let content_unit = Content::Unit;
    let result = content_unit.unexpected();
    assert_eq!(result, Unexpected::Unit);
}

#[test]
fn test_unexpected_newtype() {
    let content_newtype = Content::Newtype(Box::new(Content::Bool(false)));
    let result = content_newtype.unexpected();
    assert_eq!(result, Unexpected::NewtypeStruct);
}

#[test]
fn test_unexpected_seq() {
    let content_seq = Content::Seq(vec![Content::Bool(true), Content::String(String::from("test"))]);
    let result = content_seq.unexpected();
    assert_eq!(result, Unexpected::Seq);
}

#[test]
fn test_unexpected_map() {
    let content_map = Content::Map(vec![(Content::String(String::from("key")), Content::Bool(false))]);
    let result = content_map.unexpected();
    assert_eq!(result, Unexpected::Map);
}

