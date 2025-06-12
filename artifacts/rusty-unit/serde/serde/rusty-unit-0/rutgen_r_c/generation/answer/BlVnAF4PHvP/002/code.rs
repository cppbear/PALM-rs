// Answer 0

#[test]
fn test_deserialize_enum_valid_variant() {
    use crate::de::Visitor;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'de>;

        fn visit_enum<V>(self, _value: EnumDeserializer<'de, ()>) -> Result<V::Value, ()> {
            Ok(Content::Bool(true)) // Mocking a return type to simulate visit.
        }

        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok(Content::Unit)
        }

        // Other required visitor methods can be implemented as needed,
        // but for this test, we only need visit_enum and visit_unit.
    }

    let mut entries = vec![
        Some((Content::Str("variant1"), Content::String("value1".to_string()))),
        Some((Content::Str("variant2"), Content::String("value2".to_string()))),
    ];

    let mut deserializer = FlatMapDeserializer(&mut entries, PhantomData);

    let variants = ["variant1", "variant2"];
    let result = deserializer.deserialize_enum("TestEnum", &variants, TestVisitor);

    assert!(result.is_ok());
}

#[test]
fn test_deserialize_enum_no_variant_found() {
    use crate::de::Visitor;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Content<'de>;

        fn visit_enum<V>(self, _value: EnumDeserializer<'de, ()>) -> Result<V::Value, ()> {
            Ok(Content::Bool(true)) // Mocking a return type.
        }

        fn visit_unit(self) -> Result<Self::Value, ()> {
            Ok(Content::Unit)
        }
    }

    let mut entries = vec![
        Some((Content::Str("variant3"), Content::String("value3".to_string()))),
        Some((Content::Str("variant4"), Content::String("value4".to_string()))),
    ];

    let mut deserializer = FlatMapDeserializer(&mut entries, PhantomData);

    let variants = ["variant1", "variant2"];
    let result = deserializer.deserialize_enum("TestEnum", &variants, TestVisitor);

    assert!(result.is_err());
}

