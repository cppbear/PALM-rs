// Answer 0

#[test]
fn test_deserialize_valid_identifier() {
    struct TestDeserializer;

    impl<'de> Deserializer<'de> for TestDeserializer {
        type Error = &'static str;

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_str("tag")
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };

    let result = visitor.deserialize(TestDeserializer);
    assert_eq!(result, Ok(TagOrContentField::Tag));
}

#[test]
fn test_deserialize_invalid_identifier() {
    struct TestDeserializer;

    impl<'de> Deserializer<'de> for TestDeserializer {
        type Error = &'static str;

        fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            visitor.visit_str("unknown")
        }
    }

    let visitor = TagOrContentFieldVisitor {
        tag: "tag",
        content: "content",
    };

    let result = visitor.deserialize(TestDeserializer);
    assert!(result.is_err());
}

