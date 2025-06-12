// Answer 0

#[test]
fn test_deserialize_any_borrowed_str() {
    use std::borrow::Cow;
    use serde_json::de::{self, Visitor};
    use serde_json::Error;

    struct TestVisitor;

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(value.to_owned())
        }

        // Implement other required visitor methods as no-ops
        fn visit_string<E>(self, _: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unimplemented!()
        }

        fn visit_bytes<E>(self, _: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unimplemented!()
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unimplemented!()
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            unimplemented!()
        }

        fn visit_some<V>(self, _: V) -> Result<Self::Value, V::Error>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        // Add any additional required methods as no-ops
    }

    struct TestDeserializer {
        value: Cow<'static, str>,
    }

    impl TestDeserializer {
        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
        where
            V: Visitor<'de>,
        {
            match self.value {
                Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
                #[cfg(any(feature = "std", feature = "alloc"))]
                Cow::Owned(string) => visitor.visit_string(string),
                #[cfg(not(any(feature = "std", feature = "alloc")))]
                Cow::Owned(_) => unreachable!(),
            }
        }
    }

    let deserializer = TestDeserializer {
        value: Cow::Borrowed("test string"),
    };

    let result = deserializer.deserialize_any(TestVisitor).unwrap();
    assert_eq!(result, "test string");
}

