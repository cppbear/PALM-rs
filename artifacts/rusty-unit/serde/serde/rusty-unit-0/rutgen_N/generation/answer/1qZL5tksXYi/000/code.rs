// Answer 0

#[derive(Debug)]
struct MockVisitor {
    value: Option<()>,
}

impl<'de> serde::de::Visitor<'de> for MockVisitor {
    type Value = ();

    fn visit_unit(self) -> Result<Self::Value, serde::de::Error> {
        self.value = Some(());
        Ok(())
    }

    fn visit_bool(self, _: bool) -> Result<Self::Value, serde::de::Error> {
        Err(serde::de::Error::custom("Expected Unit"))
    }
}

struct MockDeserializer {
    content: &'static Content,
}

impl MockDeserializer {
    fn invalid_type<V>(&self, _: &V) -> serde::de::Error {
        serde::de::Error::custom("Invalid type")
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, serde::de::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match *self.content {
            Content::Unit => visitor.visit_unit(),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

#[derive(Debug)]
enum Content {
    Unit,
    Bool,
}

#[test]
fn test_deserialize_unit_success() {
    let deserializer = MockDeserializer { content: &Content::Unit };
    let visitor = MockVisitor { value: None };
    let result = deserializer.deserialize_unit(visitor);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_deserialize_unit_failure() {
    let deserializer = MockDeserializer { content: &Content::Bool };
    let visitor = MockVisitor { value: None };
    let result = deserializer.deserialize_unit(visitor);
    assert!(result.is_err());
}

