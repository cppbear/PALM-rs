// Answer 0

#[test]
fn test_unexpected_with_u32() {
    let content = Content::U32(42);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Unsigned(42));
}

#[test]
fn test_unexpected_with_u64() {
    let content = Content::U64(100);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Unsigned(100));
}

#[test]
fn test_unexpected_with_bool() {
    let content = Content::Bool(true);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Bool(true));
}

#[test]
fn test_unexpected_with_i32() {
    let content = Content::I32(-10);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Signed(-10));
}

#[test]
fn test_unexpected_with_f32() {
    let content = Content::F32(3.14);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Float(3.14));
}

#[test]
fn test_unexpected_with_string() {
    let content = Content::String("Hello".to_string());
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Str("Hello"));
}

#[test]
fn test_unexpected_with_none() {
    let content = Content::None;
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Option);
}

#[test]
fn test_unexpected_with_seq() {
    let content = Content::Seq(vec![Content::Bool(false), Content::U8(255)]);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Seq);
}

#[test]
fn test_unexpected_with_map() {
    let content = Content::Map(vec![(Content::Str("key"), Content::Str("value"))]);
    let unexpected_value = content.unexpected();
    assert_eq!(unexpected_value, Unexpected::Map);
}

