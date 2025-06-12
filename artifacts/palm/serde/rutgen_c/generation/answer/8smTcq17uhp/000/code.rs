// Answer 0

#[test]
fn test_deserialize_any_borrowed() {
    use std::borrow::Cow;

    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let deserializer = CowStrDeserializer {
        value: Cow::Borrowed("test"),
        marker: PhantomData,
    };

    let result: Result<String, _> = deserializer.deserialize_any(MockVisitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_any_owned() {
    use std::borrow::Cow;

    struct MockVisitor;

    impl<'de> de::Visitor<'de> for MockVisitor {
        type Value = String;

        fn visit_str<E>(self, value: &'de str) -> Result<Self::Value, E> {
            Ok(value.to_owned())
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
            Ok(value)
        }
    }

    let deserializer = CowStrDeserializer {
        value: Cow::Owned("owned".to_string()),
        marker: PhantomData,
    };

    let result: Result<String, _> = deserializer.deserialize_any(MockVisitor);
    assert_eq!(result.unwrap(), "owned");
}

