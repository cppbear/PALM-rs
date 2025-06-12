// Answer 0

#[test]
fn test_deserialize_any_success() {
    struct TestVisitor {
        visit_count: usize,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            // Suppose we simulate a map visit
            self.visit_count += 1;
            Ok("visited".to_string())
        }
    }

    struct TestDeserializer {
        len: usize,
        iter: Vec<usize>,
    }

    impl TestDeserializer {
        fn new(len: usize) -> Self {
            TestDeserializer {
                len,
                iter: (0..len).collect(),
            }
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    impl<'de> serde::Deserializer<'de> for TestDeserializer {
        type Error = serde::de::value::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let len = self.len();
            let mut deserializer = MapRefDeserializer::new(self);
            let map = visitor.visit_map(&mut deserializer)?;
            let remaining = deserializer.iter.len();
            if remaining == 0 {
                Ok(map)
            } else {
                Err(serde::de::Error::invalid_length(len, &"fewer elements in map"))
            }
        }
    }

    let deserializer = TestDeserializer::new(3);
    let visitor = TestVisitor { visit_count: 0 };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.ok(), Some("visited".to_string()));
}

