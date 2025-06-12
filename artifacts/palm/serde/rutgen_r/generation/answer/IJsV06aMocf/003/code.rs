// Answer 0

#[derive(Debug)]
struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
        Ok(())
    }

    fn visit_any<V>(self, _visitor: V) -> Result<Self::Value, serde::de::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("visit_any called"))
    }
}

struct Deserializer {
    content: Content,
}

enum Content {
    Map(std::collections::HashMap<String, String>),
    Seq(Vec<String>),
}

impl Deserializer {
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        Err(serde::de::Error::custom("deserialize_any called"))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.content {
            Content::Map(ref v) if v.is_empty() => visitor.visit_unit(),
            Content::Seq(ref v) if v.is_empty() => visitor.visit_unit(),
            _ => self.deserialize_any(visitor),
        }
    }
}

#[test]
fn test_deserialize_unit_struct_non_empty_map() {
    let visitor = Visitor;
    let content = Content::Map(std::collections::HashMap::from([("key".to_string(), "value".to_string())]));

    let deserializer = Deserializer { content };

    let result = deserializer.deserialize_unit_struct("TestStruct", visitor);
    assert!(result.is_err());
}

#[test]
fn test_deserialize_unit_struct_non_empty_seq() {
    let visitor = Visitor;
    let content = Content::Seq(vec!["value1".to_string(), "value2".to_string()]);

    let deserializer = Deserializer { content };

    let result = deserializer.deserialize_unit_struct("TestStruct", visitor);
    assert!(result.is_err());
}

