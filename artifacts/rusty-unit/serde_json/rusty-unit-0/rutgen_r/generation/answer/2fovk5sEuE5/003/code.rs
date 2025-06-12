// Answer 0

#[derive(Default)]
struct MockDeserializer {
    de: DeContext,
}

#[derive(Default)]
struct DeContext {
    read: MockRead,
    scratch: String,
}

struct MockRead {
    input: &'static str,
}

impl MockRead {
    fn parse_str(&mut self, output: &mut String) -> Result<Reference<'static>, &'static str> {
        if self.input.is_empty() {
            return Err("Empty input");
        }
        *output = self.input.to_string();
        self.input = ""; // simulate consuming the input
        Ok(Reference::Borrowed(&output))
    }
}

#[derive(Debug)]
enum Reference<'a> {
    Borrowed(&'a str),
    Copied(String),
}

impl MockDeserializer {
    fn deserialize_any<V>(&mut self, visitor: V) -> Result<V::Value, &'static str>
    where
        V: Visitor<'static>,
    {
        self.de.scratch.clear();
        match self.de.read.parse_str(&mut self.de.scratch) {
            Ok(Reference::Borrowed(s)) => visitor.visit_borrowed_str(s),
            Ok(Reference::Copied(s)) => visitor.visit_str(s),
            Err(err) => Err(err),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value, &'static str>;
    fn visit_str(self, v: String) -> Result<Self::Value, &'static str>;
}

struct MockVisitor {
    value: Option<String>,
}

impl<'de> Visitor<'de> for MockVisitor {
    type Value = String;

    fn visit_borrowed_str(self, v: &'de str) -> Result<Self::Value, &'static str> {
        Ok(v.to_string())
    }

    fn visit_str(self, v: String) -> Result<Self::Value, &'static str> {
        Ok(v)
    }
}

#[test]
fn test_deserialize_any_borrowed() {
    let mut deserializer = MockDeserializer {
        de: DeContext {
            read: MockRead { input: "test" },
            scratch: String::new(),
        },
    };
    let visitor = MockVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);
    assert_eq!(result.unwrap(), "test");
}

#[test]
fn test_deserialize_any_empty_input() {
    let mut deserializer = MockDeserializer {
        de: DeContext {
            read: MockRead { input: "" },
            scratch: String::new(),
        },
    };
    let visitor = MockVisitor { value: None };
    let result = deserializer.deserialize_any(visitor);
    assert!(result.is_err());
}

