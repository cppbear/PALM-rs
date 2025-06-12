// Answer 0

#[test]
fn test_deserialize_unit_with_unit_content() {
    struct TestVisitor {
        value: Option<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit value")
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::Unit,
        err: PhantomData,
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_unit_with_empty_map_content() {
    struct TestVisitor {
        value: Option<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit value")
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::Map(vec![]),
        err: PhantomData,
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_ok());
}

#[test]
fn test_deserialize_unit_with_non_unit_content() {
    struct TestVisitor {
        value: Option<()>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, Self::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit value")
        }
    }

    let deserializer = ContentDeserializer {
        content: Content::Bool(true),
        err: PhantomData,
    };
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_err());
}

