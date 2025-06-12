// Answer 0

#[test]
fn test_serialize_unit_variant() {
    struct TestSerializer;

    impl serde::Serializer for TestSerializer {
        type Ok = Value;
        type Error = Error;
        type SerializeSeq = SerializeVec;
        type SerializeTuple = SerializeVec;
        type SerializeTupleStruct = SerializeVec;
        type SerializeTupleVariant = SerializeTupleVariant;
        type SerializeMap = SerializeMap;
        type SerializeStruct = SerializeMap;
        type SerializeStructVariant = SerializeStructVariant;

        fn serialize_str(self, value: &str) -> Result<Value> {
            Ok(Value::String(value.to_owned()))
        }

        // Other methods are omitted for simplicity.
    }

    let serializer = TestSerializer;

    // Test with a valid variant name.
    let result = serializer.serialize_unit_variant("ExampleEnum", 0, "VariantA");
    assert_eq!(result.unwrap(), Value::String("VariantA".to_string()));

    // Test with another valid variant name.
    let result = serializer.serialize_unit_variant("ExampleEnum", 1, "VariantB");
    assert_eq!(result.unwrap(), Value::String("VariantB".to_string()));

    // Test with an empty variant name (still valid).
    let result = serializer.serialize_unit_variant("ExampleEnum", 2, "");
    assert_eq!(result.unwrap(), Value::String("".to_string()));
}

