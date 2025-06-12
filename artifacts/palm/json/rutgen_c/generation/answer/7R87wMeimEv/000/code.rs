// Answer 0

#[test]
fn test_deserialize_any_empty_map() {
    use serde::de::IntoDeserializer;

    let map: Map<String, Value> = Map::new();
    let result: Result<Vec<String>, Error> = map.deserialize_any(DummyVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Vec::<String>::new());
}

#[test]
fn test_deserialize_any_single_element_map() {
    use serde::de::IntoDeserializer;

    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));

    let result: Result<Vec<(String, String)>, Error> = map.deserialize_any(DummyVisitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![("key".to_string(), "value".to_string())]);
}

#[test]
fn test_deserialize_any_fewer_elements() {
    use serde::de::IntoDeserializer;

    let mut map: Map<String, Value> = Map::new();
    map.insert("key".to_string(), Value::String("value".to_string()));

    let result: Result<Vec<(String, String)>, Error> = map.deserialize_any(DummyVisitorFewerElements);
    assert!(result.is_err());
}

struct DummyVisitor;

impl<'de> serde::de::Visitor<'de> for DummyVisitor {
    type Value = Vec<(String, String)>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut vec = Vec::new();
        while let Some((key, value)) = map.next_entry::<String, String>()? {
            vec.push((key, value));
        }
        Ok(vec)
    }
}

struct DummyVisitorFewerElements;

impl<'de> serde::de::Visitor<'de> for DummyVisitorFewerElements {
    type Value = Vec<(String, String)>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map with more elements")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut vec = Vec::new();
        if let Some((key, value)) = map.next_entry::<String, String>()? {
            vec.push((key, value));
        }
        // Expecting more elements to trigger the error.
        Err(serde::de::Error::invalid_value(serde::de::Unexpected::Map, &"expecting more elements"))
    }
}

