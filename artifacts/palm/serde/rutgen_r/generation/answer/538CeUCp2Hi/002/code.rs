// Answer 0

fn test_deserialize_seq_valid() {
    struct TestVisitor {
        value: Vec<i32>,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut values = vec![];
            while let Some(value) = seq.next_element()? {
                values.push(value);
            }
            Ok(values)
        }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn new(content: Content) -> Self {
            TestContent { content }
        }

        fn invalid_type(&self, _visitor: &dyn serde::de::Visitor<'_>) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let content = Content::Seq(vec![1, 2, 3]);
    let deserializer = TestContent::new(content);
    let visitor = TestVisitor { value: vec![] };

    let result = deserializer.deserialize_seq(visitor).unwrap();

    assert_eq!(result, vec![1, 2, 3]);
}

fn test_deserialize_seq_invalid() {
    struct TestVisitor;

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence")
        }

        fn visit_seq<S>(self, _seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            Ok(())
        }
    }

    struct TestContent {
        content: Content,
    }

    impl TestContent {
        fn new(content: Content) -> Self {
            TestContent { content }
        }

        fn invalid_type(&self, _visitor: &dyn serde::de::Visitor<'_>) -> serde::de::Error {
            serde::de::Error::custom("Invalid type")
        }

        fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.content {
                Content::Seq(v) => visit_content_seq(v, visitor),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
    }

    let content = Content::OtherType; // Assume this is not seq
    let deserializer = TestContent::new(content);
    let visitor = TestVisitor;

    let result = deserializer.deserialize_seq(visitor);

    assert!(result.is_err());
}

fn visit_content_seq<V>(v: Vec<i32>, visitor: V) -> Result<V::Value, V::Error>
where
    V: serde::de::Visitor<'de>,
{
    // Mocking the visitation process on the provided sequence
    let mut seq = v.into_iter();
    visitor.visit_seq(seq.map(Some))
}

