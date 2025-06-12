// Answer 0

#[test]
fn test_deserialize_any_with_valid_map() {
    use serde::de::{self, MapAccess, Visitor};
    use serde::Deserialize;
    use std::collections::HashMap;

    struct TestDeserializer {
        map: HashMap<String, String>,
    }

    impl TestDeserializer {
        fn new(map: HashMap<String, String>) -> Self {
            TestDeserializer { map }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'static>,
        {
            visitor.visit_map(self.map.into_iter().map(|(k, v)| (k, v)))
        }
    }

    struct TestVisitor {
        visited: Vec<(String, String)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<(String, String)>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of string to string")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, <Self as Visitor<'de>>::Error>
        where
            M: MapAccess<'de>,
        {
            let mut result = Vec::new();
            while let Some((key, value)) = map.next_entry::<String, String>()? {
                result.push((key, value));
            }
            Ok(result)
        }
    }

    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let deserializer = TestDeserializer::new(map);
    let visitor = TestVisitor { visited: Vec::new() };
    
    let result = deserializer.deserialize_any(visitor).unwrap();
    
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], ("key1".to_string(), "value1".to_string()));
    assert_eq!(result[1], ("key2".to_string(), "value2".to_string()));
}

#[test]
#[should_panic]
fn test_deserialize_any_with_empty_map() {
    use serde::de::{self, MapAccess, Visitor};
    use std::collections::HashMap;

    struct TestDeserializer {
        map: HashMap<String, String>,
    }

    impl TestDeserializer {
        fn new(map: HashMap<String, String>) -> Self {
            TestDeserializer { map }
        }

        fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
        where
            V: Visitor<'static>,
        {
            visitor.visit_map(self.map.into_iter().map(|(k, v)| (k, v)))
        }
    }

    struct TestVisitor {
        visited: Vec<(String, String)>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<(String, String)>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of string to string")
        }

        fn visit_map<M>(self, _: M) -> Result<Self::Value, <Self as Visitor<'de>>::Error>
        where
            M: MapAccess<'de>,
        {
            panic!("Visiting an empty map should not happen");
        }
    }

    let deserializer = TestDeserializer::new(HashMap::new());
    let visitor = TestVisitor { visited: Vec::new() };

    let _ = deserializer.deserialize_any(visitor).unwrap();
}

