// Answer 0

#[test]
fn test_content_deserializer_new_bool() {
    let content = Content::Bool(true);
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::Bool(true)));
}

#[test]
fn test_content_deserializer_new_u8() {
    let content = Content::U8(255);
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::U8(255)));
}

#[test]
fn test_content_deserializer_new_u16() {
    let content = Content::U16(65535);
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::U16(65535)));
}

#[test]
fn test_content_deserializer_new_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)]);
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::Seq(vec![Content::U8(1), Content::U8(2), Content::U8(3)])));
}

#[test]
fn test_content_deserializer_new_map() {
    let content = Content::Map(vec![
        (Content::Str("key1"), Content::U8(10)),
        (Content::Str("key2"), Content::U8(20))
    ]);
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::Map(vec![
        (Content::Str("key1"), Content::U8(10)),
        (Content::Str("key2"), Content::U8(20))
    ])));
}

#[test]
fn test_content_deserializer_new_none() {
    let content = Content::None;
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::None));
}

#[test]
fn test_content_deserializer_new_some() {
    let content = Content::Some(Box::new(Content::U8(42)));
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::Some(box Content::U8(42))));
}

#[test]
fn test_content_deserializer_new_newtype() {
    let content = Content::NewtypeStruct("MyNewtype", Box::new(Content::Str("value")));
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::NewtypeStruct("MyNewtype", box Content::Str("value"))));
}

#[should_panic]
fn test_content_deserializer_new_empty_map() {
    let content = Content::Map(vec![]);
    let deserializer: ContentDeserializer<()> = ContentDeserializer::new(content);
    assert!(matches!(deserializer.content, Content::Map(vec![])));
}

