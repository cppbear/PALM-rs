// Answer 0

#[test]
fn test_unexpected_unit() {
    let content = Content::Unit;
    let result = content.unexpected();
}

#[test]
fn test_unexpected_bool() {
    let content = Content::Bool(true);
    let result = content.unexpected();
}

#[test]
fn test_unexpected_u8() {
    let content = Content::U8(255);
    let result = content.unexpected();
}

#[test]
fn test_unexpected_i16() {
    let content = Content::I16(-32768);
    let result = content.unexpected();
}

#[test]
fn test_unexpected_f32() {
    let content = Content::F32(3.14);
    let result = content.unexpected();
}

#[test]
fn test_unexpected_char() {
    let content = Content::Char('a');
    let result = content.unexpected();
}

#[test]
fn test_unexpected_string() {
    let content = Content::String("test".to_string());
    let result = content.unexpected();
}

#[test]
fn test_unexpected_seq() {
    let content = Content::Seq(vec![Content::Bool(false), Content::U32(42)]);
    let result = content.unexpected();
}

#[test]
fn test_unexpected_map() {
    let content = Content::Map(vec![(Content::String("key".to_string()), Content::U64(100))]);
    let result = content.unexpected();
}

