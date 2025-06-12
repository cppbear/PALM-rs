// Answer 0

#[test]
fn test_into_deserializer_bool() {
    let content = Content::Bool(true);
    let deserializer = (&content).into_deserializer();
    assert_eq!(deserializer.content, &content);
}

#[test]
fn test_into_deserializer_u8() {
    let content = Content::U8(42);
    let deserializer = (&content).into_deserializer();
    assert_eq!(deserializer.content, &content);
}

#[test]
fn test_into_deserializer_string() {
    let content = Content::String("Hello".to_string());
    let deserializer = (&content).into_deserializer();
    assert_eq!(deserializer.content, &content);
}

#[test]
fn test_into_deserializer_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer = (&content).into_deserializer();
    assert_eq!(deserializer.content, &content);
}

#[test]
fn test_into_deserializer_map() {
    let content = Content::Map(vec![
        (Content::Str("key1"), Content::U32(1)),
        (Content::Str("key2"), Content::U32(2)),
    ]);
    let deserializer = (&content).into_deserializer();
    assert_eq!(deserializer.content, &content);
}

#[test]
fn test_into_deserializer_none() {
    let content = Content::None;
    let deserializer = (&content).into_deserializer();
    assert_eq!(deserializer.content, &content);
}

