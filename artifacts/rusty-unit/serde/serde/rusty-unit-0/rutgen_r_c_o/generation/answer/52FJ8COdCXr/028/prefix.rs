// Answer 0

#[test]
fn test_serialize_unit_variant_valid() {
    let content = Content::UnitVariant("valid_string", 0, "valid_variant");
    let serializer = MySerializer::new();
    content.serialize(serializer).unwrap();
}

#[test]
#[should_panic]
fn test_serialize_unit_variant_invalid_unique() {
    let content = Content::UnitVariant("invalid_string", 1, "invalid_variant");
    let serializer = MySerializer::new();
    content.serialize(serializer).unwrap();
}

