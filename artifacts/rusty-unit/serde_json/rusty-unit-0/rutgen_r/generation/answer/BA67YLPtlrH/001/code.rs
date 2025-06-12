// Answer 0

#[test]
fn test_deserialize_any_with_owned_string() {
    use std::borrow::Cow;
    use serde_json::de::{self, Visitor};
    use serde_json::Error;

    struct TestVisitor {
        value: Option<String>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_borrowed_str<E>(self, _: &'de str) -> Result<Self::Value, E> {
            Err(de::Error::custom("should not receive borrowed str"))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let owned_string = Cow::Owned(String::from("test string"));
    struct TestDeserializer {
        value: Cow<str>,
    }

    impl TestDeserializer {
        fn new(value: Cow<str>) -> Self {
            TestDeserializer { value }
        }

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

    let deserializer = TestDeserializer::new(owned_string);
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_any(visitor).expect("Deserialization failed");

    assert_eq!(result, "test string");
}

