// Answer 0

#[test]
fn test_deserialize_seq_valid_sequence() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }

        fn visit_seq<A>(self, _: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            Ok(vec![1, 2, 3])
        }
    }

    struct MockDeserializer {
        content: Content,
    }

    impl MockDeserializer {
        fn invalid_type<V>(&self, _: &V) -> String {
            "invalid type".to_string()
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, String>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        Seq(Vec<i32>),
    }

    fn visit_content_seq<V>(content: Vec<i32>, visitor: V) -> Result<V::Value, String>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(serde::de::SeqAccess::from_iter(content.into_iter()))
    }

    let deserializer = MockDeserializer {
        content: Content::Seq(vec![1, 2, 3]),
    };
    let result: Result<Vec<i32>, _> = deserializer.deserialize_seq(MockVisitor);
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_seq_invalid_type() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }
    }

    struct MockDeserializer {
        content: Content,
    }

    impl MockDeserializer {
        fn invalid_type<V>(&self, _: &V) -> String {
            "invalid type".to_string()
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, String>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    enum Content {
        Seq(Vec<i32>),
        Other,
    }

    fn visit_content_seq<V>(content: Vec<i32>, visitor: V) -> Result<V::Value, String>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_seq(serde::de::SeqAccess::from_iter(content.into_iter()))
    }

    let deserializer = MockDeserializer {
        content: Content::Other,
    };
    let result: Result<Vec<i32>, _> = deserializer.deserialize_seq(MockVisitor);
    assert_eq!(result.unwrap_err(), "invalid type");
}

