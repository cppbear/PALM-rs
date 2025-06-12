// Answer 0

#[test]
fn test_deserialize_any_returns_err_on_visit_map_error() {
    struct MockVisitor;

    impl<'de> serde::de::Visitor<'de> for MockVisitor {
        type Value = ();

        fn visit_map<V>(self, _visitor: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>
        {
            Err(serde::de::Error::custom("visit map error"))
        }
    }

    struct MockDeserializer {
        len: usize,
    }

    impl MockDeserializer {
        fn new(len: usize) -> Self {
            Self { len }
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    let deserializer = MockDeserializer::new(1);
    let visitor = MockVisitor;

    let result: Result<(), _> = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "visit map error");
}

