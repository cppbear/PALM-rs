// Answer 0

#[derive(Debug)]
enum Content<'de> {
    None,
    Some(&'de str),
    Unit,
}

struct ContentDeserializer<'de> {
    value: &'de str,
}

impl<'de> ContentDeserializer<'de> {
    fn new(value: &'de str) -> Self {
        ContentDeserializer { value }
    }
}

struct MyVisitor<'de> {
    result: Option<&'de str>,
}

impl<'de> Visitor<'de> for MyVisitor<'de> {
    type Value = Option<&'de str>;
    type Error = &'static str;

    fn visit_none(self) -> Result<Self::Value, Self::Error> {
        Ok(None)
    }

    fn visit_some(self, _: ContentDeserializer<'de>) -> Result<Self::Value, Self::Error> {
        Ok(self.result)
    }

    fn visit_unit(self) -> Result<Self::Value, Self::Error> {
        Ok(Some("unit"))
    }
}

struct Deserializer<'de> {
    content: Content<'de>,
}

impl<'de> Deserializer<'de> {
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::None => visitor.visit_none(),
            Content::Some(v) => visitor.visit_some(ContentDeserializer::new(v)),
            Content::Unit => visitor.visit_unit(),
            _ => visitor.visit_some(self),
        }
    }
}

#[test]
fn test_deserialize_option_some() {
    let content = Content::Some("test");
    let deserializer = Deserializer { content };
    let visitor = MyVisitor { result: Some("test") };

    let result = deserializer.deserialize_option(visitor).expect("should not panic");
    assert_eq!(result, Some("test"));
}

#[test]
fn test_deserialize_option_unit() {
    let content = Content::Unit;
    let deserializer = Deserializer { content };
    let visitor = MyVisitor { result: None };

    let result = deserializer.deserialize_option(visitor).expect("should not panic");
    assert_eq!(result, Some("unit"));
}

#[test]
fn test_deserialize_option_none() {
    let content = Content::None;
    let deserializer = Deserializer { content };
    let visitor = MyVisitor { result: None };

    let result = deserializer.deserialize_option(visitor).expect("should not panic");
    assert_eq!(result, None);
}

