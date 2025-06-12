// Answer 0

#[derive(Default)]
struct DummyDeserializer {
    scratch: String,
    read: DummyReader,
}

#[derive(Default)]
struct DummyReader {
    input: String,
}

impl DummyReader {
    fn parse_str(&mut self, scratch: &mut String) -> Result<Reference, String> {
        if self.input.is_empty() {
            return Err("Empty input".to_string());
        }
        scratch.push_str(&self.input);
        Ok(Reference::Copied(scratch.clone()))
    }
}

impl DummyDeserializer {
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: de::Visitor<'_>,
    {
        self.eat_char();
        self.scratch.clear();
        match self.read.parse_str(&mut self.scratch) {
            Ok(Reference::Borrowed(s)) => visitor.visit_borrowed_str(s),
            Ok(Reference::Copied(s)) => visitor.visit_str(s),
            Err(err) => Err(err),
        }
    }

    fn eat_char(&self) {
        // Dummy implementation; could modify state if required
    }
}

#[derive(Debug)]
enum Reference {
    Borrowed(&'static str),
    Copied(String),
}

mod de {
    pub trait Visitor<'de> {
        type Value;

        fn visit_borrowed_str(self, val: &'de str) -> Result<Self::Value, String>;
        fn visit_str(self, val: String) -> Result<Self::Value, String>;
    }
}

#[derive(Default)]
struct TestVisitor {
    visited: Option<String>,
}

impl<'de> de::Visitor<'de> for TestVisitor {
    type Value = String;

    fn visit_borrowed_str(self, val: &'de str) -> Result<Self::Value, String> {
        Ok(val.to_string())
    }

    fn visit_str(self, val: String) -> Result<Self::Value, String> {
        self.visited = Some(val.clone());
        Ok(val)
    }
}

#[test]
fn test_deserialize_any_with_borrowed_str() {
    let deserializer = DummyDeserializer {
        read: DummyReader {
            input: "borrowed string".to_string(),
        },
        ..Default::default()
    };
    let visitor = TestVisitor::default();
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "borrowed string");
}

#[test]
fn test_deserialize_any_with_copied_str() {
    let deserializer = DummyDeserializer {
        read: DummyReader {
            input: "copied string".to_string(),
        },
        ..Default::default()
    };
    let visitor = TestVisitor::default();
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "copied string");
}

#[test]
#[should_panic(expected = "Empty input")]
fn test_deserialize_any_with_empty_input() {
    let deserializer = DummyDeserializer {
        read: DummyReader {
            input: "".to_string(),
        },
        ..Default::default()
    };
    let visitor = TestVisitor::default();
    deserializer.deserialize_any(visitor).unwrap();
}

