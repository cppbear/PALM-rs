// Answer 0

#[derive(Debug)]
struct TestDeserializer {
    content: Content,
}

#[derive(Debug)]
enum Content {
    Seq(Vec<i32>),
}

impl TestDeserializer {
    fn invalid_type<'de, V>(&self, visitor: &V) -> String {
        format!("Invalid type for visitor: {:?}", visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, String>
    where
        V: Visitor<'de>,
    {
        match self.content {
            Content::Seq(v) => visit_content_seq(v, visitor),
            _ => Err(self.invalid_type(&visitor)),
        }
    }
}

trait Visitor<'de> {
    type Value;

    fn visit_seq(self, values: Vec<i32>) -> Result<Self::Value, String>;
}

fn visit_content_seq<V>(values: Vec<i32>, visitor: V) -> Result<V::Value, String>
where
    V: Visitor<'static>,
{
    visitor.visit_seq(values)
}

struct TestVisitor {
    values: Vec<i32>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Vec<i32>;

    fn visit_seq(self, values: Vec<i32>) -> Result<Self::Value, String> {
        Ok(values)
    }
}

#[test]
fn test_deserialize_seq_valid() {
    let deserializer = TestDeserializer {
        content: Content::Seq(vec![1, 2, 3]),
    };
    let visitor = TestVisitor { values: vec![] };

    let result: Result<Vec<i32>, String> = deserializer.deserialize_seq(visitor);
    assert_eq!(result, Ok(vec![1, 2, 3]));
}

#[test]
fn test_deserialize_seq_invalid() {
    let deserializer = TestDeserializer {
        content: Content::Seq(vec![1, 2, 3]),
    };
    struct InvalidVisitor;

    impl<'de> Visitor<'de> for InvalidVisitor {
        type Value = ();

        fn visit_seq(self, _values: Vec<i32>) -> Result<Self::Value, String> {
            Err("Invalid visitor".to_string())
        }
    }

    let visitor = InvalidVisitor;
    let result: Result<(), String> = deserializer.deserialize_seq(visitor);

    assert!(result.is_err());
}

