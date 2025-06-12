// Answer 0

#[test]
fn test_enum_deserializer_new() {
    use crate::de::Content;

    // Testing with a valid variant and value
    let variant = Content::String("example".to_string());
    let value = Some(Content::U32(42));
    let deserializer: EnumDeserializer<()>;
    deserializer = EnumDeserializer::new(variant.clone(), value.clone());
    
    assert_eq!(matches!(deserializer.variant, Content::String(_)), true);
    assert_eq!(deserializer.value, value);
}

#[test]
fn test_enum_deserializer_new_none_value() {
    use crate::de::Content;

    // Testing with a valid variant and None value
    let variant = Content::U64(100);
    let value: Option<Content> = None;
    let deserializer: EnumDeserializer<()>;
    deserializer = EnumDeserializer::new(variant.clone(), value.clone());

    assert_eq!(matches!(deserializer.variant, Content::U64(_)), true);
    assert_eq!(deserializer.value, value);
}

