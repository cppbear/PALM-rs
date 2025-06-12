// Answer 0

#[cfg(test)]
fn test_serialize_unit_variant() {
    struct MockSerializer;
    
    impl MockSerializer {
        fn serialize_str(&self, value: &'static str) -> Result<()> {
            // Returning Ok to mimic successful serialization
            Ok(())
        }
    }

    let serializer = MockSerializer;

    // Test with a normal variant
    let result = serializer.serialize_unit_variant("MyEnum", 0, "VariantA");
    assert!(result.is_ok());

    // Test with a different variant
    let result = serializer.serialize_unit_variant("MyEnum", 1, "VariantB");
    assert!(result.is_ok());
}

