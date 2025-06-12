// Answer 0

#[derive(Debug)]
struct DummyContent {
    content: Content,
}

#[derive(Debug)]
enum Content {
    Seq(Vec<i32>),
    Map(std::collections::HashMap<String, i32>),
}

impl DummyContent {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }
}

#[test]
fn test_deserialize_struct_seq() {
    use serde::de::{Visitor, Deserializer};

    struct SeqVisitor;

    impl<'de> Visitor<'de> for SeqVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a sequence of integers")
        }
        
        fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
        where
            S: serde::de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(vec)
        }
    }

    let content = DummyContent {
        content: Content::Seq(vec![1, 2, 3]),
    };

    let result: Result<Vec<i32>, _> = content.deserialize_struct("test", &["field1"], SeqVisitor);

    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_deserialize_struct_map() {
    use serde::de::{Visitor, Deserializer};

    struct MapVisitor;

    impl<'de> Visitor<'de> for MapVisitor {
        type Value = std::collections::HashMap<String, i32>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of strings to integers")
        }
        
        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut hash_map = std::collections::HashMap::new();
            while let Some((key, value)) = map.next_entry()? {
                hash_map.insert(key, value);
            }
            Ok(hash_map)
        }
    }

    let content = DummyContent {
        content: Content::Map(vec![("key1".to_string(), 1)].into_iter().collect()),
    };

    let result: Result<std::collections::HashMap<String, i32>, _> = content.deserialize_struct("test", &["field1"], MapVisitor);

    assert_eq!(result.unwrap(), vec![("key1".to_string(), 1)].into_iter().collect::<std::collections::HashMap<_, _>>());
}

#[test]
#[should_panic(expected = "Invalid type")]
fn test_deserialize_struct_invalid_type() {
    let content = DummyContent {
        content: Content::Seq(vec![]),
    };

    let result: Result<Vec<i32>, _> = content.deserialize_struct("test", &["field1"], SeqVisitor);
    // This will panic due to the invalid type handling
    let _ = result.unwrap(); // Triggers panic
}

