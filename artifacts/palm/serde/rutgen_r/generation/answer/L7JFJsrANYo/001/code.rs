// Answer 0

#[derive(Debug)]
enum Content {
    Unit,
    Map(Vec<(String, String)>),
    Other,
}

struct Deserializer {
    content: Content,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'static>,
    {
        match self.content {
            Content::Unit => visitor.visit_unit(),
            Content::Map(ref v) if v.is_empty() => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;
    
    fn visit_unit(self) -> Result<Self::Value, String>;
}

struct MyVisitor;

impl Visitor<'static> for MyVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, String> {
        Ok(())
    }
}

#[test]
fn test_deserialize_unit_invalid_type() {
    let deserializer = Deserializer {
        content: Content::Other,
    };
    let visitor = MyVisitor;

    let result = deserializer.deserialize_unit(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

#[test]
fn test_deserialize_unit_non_empty_map() {
    let deserializer = Deserializer {
        content: Content::Map(vec![("key".to_string(), "value".to_string())]),
    };
    let visitor = MyVisitor;

    let result = deserializer.deserialize_unit(visitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

