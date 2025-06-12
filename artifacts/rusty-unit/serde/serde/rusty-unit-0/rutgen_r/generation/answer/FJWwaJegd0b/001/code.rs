// Answer 0

#[derive(Debug)]
struct VisitorImpl;

impl<'de> serde::de::Visitor<'de> for VisitorImpl {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(())
    }
}

#[derive(Debug)]
enum Content {
    Map(Vec<(String, String)>),
    OtherContent,
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> serde::de::Error {
        serde::de::Error::custom("Invalid type")
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Map(v) => visit_content_map(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

#[test]
fn test_deserialize_map_with_invalid_type() {
    let deserializer = Deserializer {
        content: Content::OtherContent,
    };
    let visitor = VisitorImpl;

    let result: Result<(), _> = deserializer.deserialize_map(visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_map_with_valid_map_content() {
    let deserializer = Deserializer {
        content: Content::Map(vec![("key1".to_string(), "value1".to_string())]),
    };
    let visitor = VisitorImpl;

    let result = deserializer.deserialize_map(visitor);
    assert!(result.is_err());
}

