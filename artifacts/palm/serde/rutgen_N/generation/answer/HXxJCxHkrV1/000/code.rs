// Answer 0

#[derive(Default)]
struct MockMap {
    value: Option<Content>,
}

impl MockMap {
    fn serialize_value(&self, content: &Content) -> Result<(), String> {
        // Implementation for mocking serialization
        self.value = Some(content.clone());
        Ok(())
    }
}

#[derive(Clone)]
enum Content {
    Seq(Vec<i32>),
}

struct Serializer {
    map: MockMap,
    fields: Vec<i32>,
}

impl Serializer {
    fn new(fields: Vec<i32>) -> Self {
        Self {
            map: MockMap::default(),
            fields,
        }
    }

    fn end(self) -> Result<(), String> {
        self.map.serialize_value(&Content::Seq(self.fields))?;
        Ok(())
    }
}

#[test]
fn test_end_success() {
    let serializer = Serializer::new(vec![1, 2, 3]);
    let result = serializer.end();
    assert!(result.is_ok());
}

#[test]
fn test_end_with_empty_fields() {
    let serializer = Serializer::new(vec![]);
    let result = serializer.end();
    assert!(result.is_ok());
}

