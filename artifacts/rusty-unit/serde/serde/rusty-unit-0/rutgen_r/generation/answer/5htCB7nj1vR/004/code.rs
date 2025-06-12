// Answer 0

#[derive(Debug)]
enum Content<'de> {
    None,
    Some(&'de str),
    Unit,
}

struct ContentDeserializer<'de> {
    content: Content<'de>,
}

impl<'de> ContentDeserializer<'de> {
    fn new(content: &'de str) -> Self {
        Self {
            content: Content::Some(content),
        }
    }
}

#[derive(Debug)]
struct TestVisitor<'de> {
    visited: Option<&'de str>,
}

impl<'de> Visitor<'de> for TestVisitor<'de> {
    type Value = Option<&'de str>;
    type Error = ();

    fn visit_none(self) -> Result<Self::Value, Self::Error> {
        Ok(None)
    }

    fn visit_some(self, _: ContentDeserializer<'de>) -> Result<Self::Value, Self::Error> {
        Ok(Some("some value"))
    }

    fn visit_unit(self) -> Result<Self::Value, Self::Error> {
        Ok(Some("unit value"))
    }
}

#[test]
fn test_deserialize_option_none() {
    let deserializer = ContentDeserializer {
        content: Content::None,
    };
    let visitor = TestVisitor { visited: None };
    let result = deserializer.deserialize_option(visitor);
    assert_eq!(result, Ok(None));
}

#[test]
fn test_deserialize_option_some() {
    let deserializer = ContentDeserializer {
        content: Content::Some("value"),
    };
    let visitor = TestVisitor { visited: None };
    let result = deserializer.deserialize_option(visitor);
    assert_eq!(result, Ok(Some("some value")));
}

#[test]
fn test_deserialize_option_unit() {
    let deserializer = ContentDeserializer {
        content: Content::Unit,
    };
    let visitor = TestVisitor { visited: None };
    let result = deserializer.deserialize_option(visitor);
    assert_eq!(result, Ok(Some("unit value")));
}

