// Answer 0

#[test]
fn test_new_enum_deserializer_with_bool_variant() {
    let variant = Content::Bool(true);
    let value = Some(Content::None);
    let deserializer: EnumDeserializer<dyn de::Error> = EnumDeserializer::new(variant, value);
    assert!(matches!(deserializer.variant, Content::Bool(true)));
    assert!(matches!(deserializer.value, Some(Content::None)));
}

#[test]
fn test_new_enum_deserializer_with_u8_variant() {
    let variant = Content::U8(255);
    let value = None;
    let deserializer: EnumDeserializer<dyn de::Error> = EnumDeserializer::new(variant, value);
    assert!(matches!(deserializer.variant, Content::U8(255)));
    assert!(deserializer.value.is_none());
}

#[test]
fn test_new_enum_deserializer_with_string_variant() {
    let variant = Content::String("test".to_string());
    let value = Some(Content::U32(42));
    let deserializer: EnumDeserializer<dyn de::Error> = EnumDeserializer::new(variant, value);
    assert!(matches!(deserializer.variant, Content::String(ref s) if s == "test"));
    assert!(matches!(deserializer.value, Some(Content::U32(42))));
}

#[test]
fn test_new_enum_deserializer_with_struct_variant() {
    let variant = Content::Struct("MyStruct", vec![("field1", Content::I32(1))]);
    let value = Some(Content::Seq(vec![Content::I32(2), Content::I32(3)]));
    let deserializer: EnumDeserializer<dyn de::Error> = EnumDeserializer::new(variant, value);
    assert!(matches!(deserializer.variant, Content::Struct(name, _) if name == "MyStruct"));
    assert!(matches!(deserializer.value, Some(Content::Seq(ref seq)) if seq.len() == 2));
}

#[test]
fn test_new_enum_deserializer_with_empty_variant() {
    let variant = Content::Unit;
    let value = None;
    let deserializer: EnumDeserializer<dyn de::Error> = EnumDeserializer::new(variant, value);
    assert!(matches!(deserializer.variant, Content::Unit));
    assert!(deserializer.value.is_none());
}

