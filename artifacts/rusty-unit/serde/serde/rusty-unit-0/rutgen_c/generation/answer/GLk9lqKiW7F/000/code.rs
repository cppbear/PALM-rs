// Answer 0

#[test]
fn test_borrow_cow_str_with_str() {
    struct TestDeserializer;
    impl Deserializer<'_> for TestDeserializer {
        type Error = serde_json::Error;

        fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'_>,
        {
            let visitor = CowStrVisitor;
            visitor.visit_str("test string")
        }
    }

    let result: Result<Cow<String>, _> = borrow_cow_str(TestDeserializer);
    assert_eq!(result.unwrap(), Cow::Owned("test string".to_owned()));
}

#[test]
fn test_borrow_cow_str_with_borrowed_str() {
    struct TestDeserializer;
    impl Deserializer<'_> for TestDeserializer {
        type Error = serde_json::Error;

        fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'_>,
        {
            let visitor = CowStrVisitor;
            visitor.visit_borrowed_str("borrowed string")
        }
    }

    let result: Result<Cow<str>, _> = borrow_cow_str(TestDeserializer);
    assert_eq!(result.unwrap(), Cow::Borrowed("borrowed string"));
}

#[test]
fn test_borrow_cow_str_with_bytes() {
    struct TestDeserializer;
    impl Deserializer<'_> for TestDeserializer {
        type Error = serde_json::Error;

        fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'_>,
        {
            let visitor = CowStrVisitor;
            visitor.visit_bytes(b"byte string")
        }
    }

    let result: Result<Cow<String>, _> = borrow_cow_str(TestDeserializer);
    assert_eq!(result.unwrap(), Cow::Owned("byte string".to_owned()));
}

#[test]
fn test_borrow_cow_str_with_invalid_utf8() {
    struct TestDeserializer;
    impl Deserializer<'_> for TestDeserializer {
        type Error = serde_json::Error;

        fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'_>,
        {
            let visitor = CowStrVisitor;
            visitor.visit_bytes(b"\xFF\xFE")
        }
    }

    let result: Result<Cow<String>, _> = borrow_cow_str(TestDeserializer);
    assert!(result.is_err());
}

