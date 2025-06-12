// Answer 0

#[derive(Default)]
struct MockVisitor {
    should_return_ok: bool,
}

impl<'de> de::Visitor<'de> for MockVisitor {
    type Value = u32;

    fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
    where
        V: de::SeqAccess<'de>,
    {
        if self.should_return_ok {
            Ok(42)
        } else {
            Err(V::Error::custom("visit_seq error"))
        }
    }
}

struct MockDeserializer {
    should_end_ok: bool,
}

impl MockDeserializer {
    fn new(should_end_ok: bool) -> Self {
        Self { should_end_ok }
    }

    fn end(&mut self) -> Result<(), &'static str> {
        if self.should_end_ok {
            Ok(())
        } else {
            Err("end error")
        }
    }
}

impl<'de> de::Deserializer<'de> for MockDeserializer {
    type Error = &'static str;

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let value = tri!(visitor.visit_seq(&mut self));
        tri!(self.end());
        Ok(value)
    }
}

#[test]
fn test_deserialize_seq_ok() {
    let visitor = MockVisitor { should_return_ok: true };
    let mut deserializer = MockDeserializer::new(true);

    let result = deserializer.deserialize_seq(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_seq_visit_seq_err() {
    let visitor = MockVisitor { should_return_ok: false };
    let mut deserializer = MockDeserializer::new(true);

    let result = deserializer.deserialize_seq(visitor);
    assert_eq!(result, Err("visit_seq error"));
}

#[test]
fn test_deserialize_seq_end_err() {
    let visitor = MockVisitor { should_return_ok: true };
    let mut deserializer = MockDeserializer::new(false);

    let result = deserializer.deserialize_seq(visitor);
    assert_eq!(result, Err("end error"));
}

