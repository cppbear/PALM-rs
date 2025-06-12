// Answer 0

#[derive(Debug)]
enum Content {
    Invalid,
}

struct Deserializer {
    content: Box<Content>,
}

impl Deserializer {
    fn invalid_type<V>(&self, _visitor: &V) -> String {
        "Invalid type".to_string()
    }

    fn deserialize_integer<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor,
    {
        match *self.content {
            Content::Invalid => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor {
    type Value;
}

struct TestVisitor;

impl Visitor for TestVisitor {
    type Value = ();
}

#[test]
fn test_deserialize_integer_invalid_type() {
    let deserializer = Deserializer {
        content: Box::new(Content::Invalid),
    };
    let visitor = TestVisitor;

    let result: Result<(), String> = deserializer.deserialize_integer(visitor);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid type");
}

