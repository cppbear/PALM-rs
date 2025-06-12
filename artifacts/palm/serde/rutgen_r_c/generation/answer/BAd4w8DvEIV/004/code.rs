// Answer 0

#[test]
fn test_struct_variant_none() {
    struct TestError;

    impl de::Error for TestError {
        fn invalid_type(_unexpected: Unexpected, _expected: &'static str) -> Self {
            TestError
        }
    }

    struct TestVisitor;

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _: V) -> Result<Self::Value, Self::Error>
        where
            V: de::MapAccess<'de>,
        {
            Err(TestError)
        }

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Self::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Err(TestError)
        }
    }

    let deserializer: VariantRefDeserializer<TestError> = VariantRefDeserializer {
        value: None,
        err: PhantomData,
    };

    let result: Result<(), TestError> = deserializer.struct_variant(&[], TestVisitor);

    assert!(result.is_err());
}

