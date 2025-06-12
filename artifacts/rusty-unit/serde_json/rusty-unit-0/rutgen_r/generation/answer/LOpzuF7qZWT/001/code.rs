// Answer 0

#[test]
fn test_deserialize_ignored_any() {
    use serde::de::{self, Visitor};

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            Ok(())
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit")
        }
    }

    struct TestDeserializer;

    impl<'de> TestDeserializer {
        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de>,
        {
            drop(self);
            visitor.visit_unit()
        }
    }

    let deserializer = TestDeserializer;
    let visitor = TestVisitor;

    let result = deserializer.deserialize_ignored_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ());
}

#[test]
#[should_panic]
fn test_deserialize_ignored_any_panic() {
    use serde::de::{self, Visitor};

    struct PanicVisitor;

    impl<'de> Visitor<'de> for PanicVisitor {
        type Value = ();

        fn visit_unit(self) -> Result<Self::Value, de::Error> {
            panic!("Intentional panic in visitor");
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("unit")
        }
    }

    struct TestDeserializer;

    impl<'de> TestDeserializer {
        fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, de::Error>
        where
            V: Visitor<'de>,
        {
            drop(self);
            visitor.visit_unit()
        }
    }

    let deserializer = TestDeserializer;
    let visitor = PanicVisitor;

    let _ = deserializer.deserialize_ignored_any(visitor);
}

