// Answer 0

#[test]
fn test_into_deserializer() {
    let content = Content::Bool(true);
    let deserializer: ContentRefDeserializer<(), ()> = content.into_deserializer();
    assert_eq!(matches!(deserializer.content, &Content::Bool(true)), true);
}

#[test]
fn test_into_deserializer_string() {
    let content = Content::String("test".to_string());
    let deserializer: ContentRefDeserializer<(), ()> = content.into_deserializer();
    assert_eq!(matches!(deserializer.content, &Content::String(ref s) if s == "test"), true);
}

#[test]
fn test_into_deserializer_none() {
    let content = Content::None;
    let deserializer: ContentRefDeserializer<(), ()> = content.into_deserializer();
    assert_eq!(matches!(deserializer.content, &Content::None), true);
}

#[test]
fn test_into_deserializer_seq() {
    let content = Content::Seq(vec![Content::U8(1), Content::U8(2)]);
    let deserializer: ContentRefDeserializer<(), ()> = content.into_deserializer();
    assert_eq!(matches!(deserializer.content, Content::Seq(ref seq) if seq.len() == 2), true);
}

