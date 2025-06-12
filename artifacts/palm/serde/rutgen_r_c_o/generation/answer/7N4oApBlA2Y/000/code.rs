// Answer 0

#[derive(Debug)]
struct MockVisitor<'de> {
    content: Content<'de>,
}

impl<'de> Visitor<'de> for MockVisitor<'de> {
    type Value = Content<'de>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a Content")
    }

    fn visit_value(self, _: Content<'de>) -> Result<Self::Value, Self::Error> {
        Ok(self.content)
    }
}

#[derive(Debug)]
struct MockDeserializer<'de> {
    content: Content<'de>,
}

impl<'de> MockDeserializer<'de> {
    fn __deserialize_content<V>(
        self,
        _: actually_private::T,
        visitor: V,
    ) -> Result<Content<'de>, Self::Error>
    where
        V: Visitor<'de, Value = Content<'de>>,
    {
        let _ = visitor;
        Ok(self.content)
    }
}

#[derive(Debug)]
struct Content<'de> {
    data: &'de str,
}

impl Content<'_> {
    fn new(data: &str) -> Content {
        Content { data }
    }
}

#[test]
fn test_deserialize_content() {
    let content = Content::new("test data");
    let deserializer = MockDeserializer { content };
    let visitor = MockVisitor { content: Content::new("visited data") };

    let result = deserializer.__deserialize_content(actually_private::T, visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().data, "visited data");
}

