// Answer 0

#[test]
fn test_deserialize_option_none() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(()))
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(()))
        }
    }

    let deserializer = ContentDeserializer::new(Content::None);
    let result = deserializer.deserialize_option(TestVisitor);
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_deserialize_option_some() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u8>;

        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(5))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(None)
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(()))
        }
    }

    let deserializer = ContentDeserializer::new(Content::Some(Box::new(Content::U8(5))));
    let result = deserializer.deserialize_option(TestVisitor);
    assert_eq!(result.unwrap(), Some(5));
}

#[test]
fn test_deserialize_option_unit() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<()>;

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(()))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(None)
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(()))
        }
    }

    let deserializer = ContentDeserializer::new(Content::Unit);
    let result = deserializer.deserialize_option(TestVisitor);
    assert_eq!(result.unwrap(), Some(()));
}

#[test]
fn test_deserialize_option_invalid() {
    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Option<u8>;

        fn visit_some<V>(self, _: V) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(5))
        }

        fn visit_none(self) -> Result<Self::Value, serde::de::Error> {
            Ok(None)
        }

        fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
            Ok(Some(()))
        }
    }

    let deserializer = ContentDeserializer::new(Content::I32(10)); // Invalid type for Option
    let result = deserializer.deserialize_option(TestVisitor);
    assert!(result.is_err());
}

