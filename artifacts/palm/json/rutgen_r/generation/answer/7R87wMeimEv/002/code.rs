// Answer 0

#[test]
fn test_deserialize_any_ok() {
    struct TestVisitor {
        value: Option<String>,
    }
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok("deserialized_value".to_string())
        }
    }

    struct TestDeserializer {
        len: usize,
        iter: Vec<i32>,
    }

    impl TestDeserializer {
        fn new(len: usize) -> Self {
            Self {
                len,
                iter: vec![1, 2, 3], // This simulates extra elements
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }

    impl<'de> serde::de::Deserializer<'de> for TestDeserializer {
        type Error = serde::de::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let len = self.len();
            let mut deserializer = self; // Using self as deserializer
            let map = visitor.visit_map(&mut deserializer)?;
            let remaining = deserializer.iter.len();
            if remaining == 0 {
                Ok(map)
            } else {
                Err(serde::de::Error::invalid_length(
                    len,
                    &"fewer elements in map",
                ))
            }
        }
    }

    let deserializer = TestDeserializer::new(3);
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_any_err() {
    struct TestVisitor {
        value: Option<String>,
    }
    
    impl<'de> serde::de::Visitor<'de> for TestVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map")
        }

        fn visit_map<M>(self, _map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            Ok("some_value".to_string())
        }
    }

    struct TestDeserializer {
        len: usize,
        iter: Vec<i32>,
    }

    impl TestDeserializer {
        fn new(len: usize) -> Self {
            Self {
                len,
                iter: vec![], // This simulates no extra elements
            }
        }
        
        fn len(&self) -> usize {
            self.len
        }
    }

    impl<'de> serde::de::Deserializer<'de> for TestDeserializer {
        type Error = serde::de::Error;

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: serde::de::Visitor<'de>,
        {
            let len = self.len();
            let mut deserializer = self; // Using self as deserializer
            let map = visitor.visit_map(&mut deserializer)?;
            let remaining = deserializer.iter.len();
            if remaining == 0 {
                Ok(map)
            } else {
                Err(serde::de::Error::invalid_length(
                    len,
                    &"fewer elements in map",
                ))
            }
        }
    }

    let deserializer = TestDeserializer::new(3);
    let visitor = TestVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

