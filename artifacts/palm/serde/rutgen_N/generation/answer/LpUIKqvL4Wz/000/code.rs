// Answer 0

#[derive(Debug)]
struct Content {
    elements: Vec<String>,
}

#[derive(Debug)]
struct Serializer {
    elements: Vec<String>,
}

impl Serializer {
    fn new() -> Self {
        Serializer {
            elements: Vec::new(),
        }
    }

    fn end(self) -> Result<Content, String> {
        Ok(Content { elements: self.elements })
    }
}

#[test]
fn test_end_with_empty_elements() {
    let serializer = Serializer::new();
    let result = serializer.end().unwrap();
    assert_eq!(result.elements.len(), 0);
}

#[test]
fn test_end_with_some_elements() {
    let mut serializer = Serializer::new();
    serializer.elements.push("element1".to_string());
    serializer.elements.push("element2".to_string());
    let result = serializer.end().unwrap();
    assert_eq!(result.elements.len(), 2);
    assert_eq!(result.elements[0], "element1");
    assert_eq!(result.elements[1], "element2");
}

