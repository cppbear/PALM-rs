// Answer 0

#[derive(Debug)]
enum Content {
    U32(u32),
}

struct Deserializer {
    content: Box<Content>,
}

impl Deserializer {
    fn new(content: Content) -> Self {
        Self {
            content: Box::new(content),
        }
    }

    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor,
    {
        match *self.content {
            Content::U32(v) => visitor.visit_u32(v),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor {
    type Value;

    fn visit_u32(self, value: u32) -> Result<Self::Value, String>;
}

struct TestVisitor {
    output: u32,
}

impl Visitor for TestVisitor {
    type Value = u32;

    fn visit_u32(self, value: u32) -> Result<Self::Value, String> {
        Ok(value)
    }
}

#[test]
fn test_deserialize_integer_u32() {
    let deserializer = Deserializer::new(Content::U32(42));
    let visitor = TestVisitor { output: 0 };
    let result = deserializer.deserialize_integer(visitor);
    assert_eq!(result, Ok(42));
}

#[test]
fn test_deserialize_integer_invalid_type() {
    struct InvalidVisitor;

    impl Visitor for InvalidVisitor {
        type Value = ();

        fn visit_u32(self, _value: u32) -> Result<Self::Value, String> {
            unreachable!()
        }
    }

    let deserializer = Deserializer::new(Content::U32(42));
    let result = deserializer.deserialize_integer(InvalidVisitor);
    assert_eq!(result, Err("Invalid type".to_string()));
}

