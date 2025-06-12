// Answer 0

#[test]
fn test_unexpected_i8() {
    let content = Content::I8(-10);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Signed(-10));
}

#[test]
fn test_unexpected_u8() {
    let content = Content::U8(10);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Unsigned(10));
}

#[test]
fn test_unexpected_f32() {
    let content = Content::F32(3.14);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Float(3.14));
}

#[test]
fn test_unexpected_char() {
    let content = Content::Char('a');
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Char('a'));
}

#[test]
fn test_unexpected_string() {
    let content = Content::String("hello".to_string());
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Str("hello"));
}

#[test]
fn test_unexpected_none() {
    let content = Content::None;
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Option);
}

#[test]
fn test_unexpected_seq() {
    let content = Content::Seq(vec![Content::I8(-1), Content::I8(2)]);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Seq);
}

#[test]
fn test_unexpected_map() {
    let content = Content::Map(vec![(Content::String("key".to_string()), Content::I8(1))]);
    let unexpected = content.unexpected();
    assert_eq!(unexpected, Unexpected::Map);
}

