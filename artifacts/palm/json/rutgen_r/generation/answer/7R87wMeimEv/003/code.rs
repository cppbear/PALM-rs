// Answer 0

#[test]
fn test_deserialize_any_with_remaining_elements() {
    struct TestVisitor {
        visited: bool,
    }

    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = ();

        fn visit_map<V>(self, _map: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            self.visited = true;
            Ok(())
        }
    }

    struct TestDeserializer {
        elements: usize,
        iter: Vec<()>,
    }

    impl TestDeserializer {
        fn new(elements: usize) -> Self {
            TestDeserializer {
                elements,
                iter: vec![(); elements],
            }
        }

        fn len(&self) -> usize {
            self.elements
        }
    }

    let len = 3;
    let mut deserializer = TestDeserializer::new(len);
    let visitor = TestVisitor { visited: false };

    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), serde::de::Error::invalid_length(len, &"fewer elements in map"));
}

