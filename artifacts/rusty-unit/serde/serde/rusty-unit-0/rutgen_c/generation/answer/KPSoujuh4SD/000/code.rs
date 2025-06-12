// Answer 0

#[test]
fn test_deserialize_i16_valid() {
    struct TestVisitor {
        value: Option<i16>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<i16>;

        fn visit_i16(self, value: i16) -> Result<Self::Value, Self::Error> {
            Ok(Some(value))
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(None)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i16 or unit value")
        }
    }

    let content = Content::I16(42);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_i16(visitor).unwrap();
    assert_eq!(result, Some(42));
}

#[test]
fn test_deserialize_i16_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_i16(self, _value: i16) -> Result<Self::Value, Self::Error> {
            panic!("should not be called");
        }

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("an i16 value")
        }
    }

    let content = Content::Bool(true);
    let deserializer = ContentRefDeserializer {
        content: &content,
        err: PhantomData,
    };
    let visitor = TestVisitor;
    let result = deserializer.deserialize_i16(visitor);
    assert!(result.is_err());
}

