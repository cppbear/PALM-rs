// Answer 0

#[derive(Debug)]
struct TestDeserializer {
    content: Content,
}

#[derive(Debug)]
enum Content {
    Map(Vec<(String, String)>),
    Other,
}

impl<'de> TestDeserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> &'static str {
        "Invalid type"
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Map(v) => visit_content_map(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;
    fn visit_map(self, values: Vec<(String, String)>) -> Result<Self::Value, &'static str>;
}

fn visit_content_map<V>(values: Vec<(String, String)>, visitor: V) -> Result<V::Value, &'static str>
where
    V: Visitor<'de>,
{
    visitor.visit_map(values)
}

struct TestVisitor {
    data: Vec<(String, String)>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Vec<(String, String)>;

    fn visit_map(self, values: Vec<(String, String)>) -> Result<Self::Value, &'static str> {
        Ok(values)
    }
}

#[test]
fn test_deserialize_map_valid() {
    let deserializer = TestDeserializer {
        content: Content::Map(vec![("key1".to_string(), "value1".to_string())]),
    };
    let visitor = TestVisitor { data: Vec::new() };
    
    let result: Result<Vec<(String, String)>, _> = deserializer.deserialize_map(visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![("key1".to_string(), "value1".to_string())]);
}

#[test]
fn test_deserialize_map_invalid() {
    let deserializer = TestDeserializer {
        content: Content::Other,
    };
    let visitor = TestVisitor { data: Vec::new() };
    
    let result: Result<Vec<(String, String)>, _> = deserializer.deserialize_map(visitor);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid type");
}

