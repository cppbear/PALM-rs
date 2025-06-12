// Answer 0

#[test]
fn test_tuple_variant_with_some_seq() {
    struct TestVisitor {
        value: Option<Vec<i32>>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut values = Vec::new();
            while let Some(value) = visitor.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    struct TestDeserializer {
        value: Option<Content>,
    }

    impl TestDeserializer {
        fn new(value: Option<Content>) -> Self {
            Self { value }
        }
    }

    impl<'de> de::Deserializer<'de> for TestDeserializer {
        type Error = de::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let len = match &self.value {
                Some(Content::Seq(seq)) => seq.len(),
                _ => 0,
            };
            self.tuple_variant(len, visitor)
        }
    }

    impl TestDeserializer {
        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                Some(Content::Seq(v)) => {
                    de::Deserializer::deserialize_any(SeqDeserializer::new(v.into_iter()), visitor)
                }
                Some(other) => Err(de::Error::invalid_type(other.unexpected(), &"tuple variant")),
                None => Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"tuple variant")),
            }
        }
    }

    #[derive(Debug)]
    enum Content {
        Seq(Vec<i32>),
    }

    let seq_content = Some(Content::Seq(vec![1, 2, 3]));
    let deserializer = TestDeserializer::new(seq_content);
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_tuple_variant_with_none() {
    struct TestVisitor {
        value: Option<Vec<i32>>,
    }

    impl<'de> de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }

        fn visit_seq<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            Ok(vec![])
        }
    }

    struct TestDeserializer {
        value: Option<Content>,
    }

    impl TestDeserializer {
        fn new(value: Option<Content>) -> Self {
            Self { value }
        }
    }

    impl<'de> de::Deserializer<'de> for TestDeserializer {
        type Error = de::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            let len = match &self.value {
                Some(Content::Seq(seq)) => seq.len(),
                _ => 0,
            };
            self.tuple_variant(len, visitor)
        }
    }

    impl TestDeserializer {
        fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                Some(Content::Seq(v)) => {
                    de::Deserializer::deserialize_any(SeqDeserializer::new(v.into_iter()), visitor)
                }
                Some(other) => Err(de::Error::invalid_type(other.unexpected(), &"tuple variant")),
                None => Err(de::Error::invalid_type(de::Unexpected::UnitVariant, &"tuple variant")),
            }
        }
    }

    #[derive(Debug)]
    enum Content {
        Seq(Vec<i32>),
    }

    let deserializer = TestDeserializer::new(None);
    let visitor = TestVisitor { value: None };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

