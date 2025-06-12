// Answer 0

#[derive(Debug)]
struct Content {
    elements: Vec<i32>,
}

impl Content {
    fn seq(elements: Vec<i32>) -> Self {
        Content { elements }
    }
}

#[derive(Debug)]
struct Serializer {
    elements: Vec<i32>,
}

impl Serializer {
    fn new() -> Self {
        Serializer { elements: vec![] }
    }

    fn end(self) -> Result<Content, &'static str> {
        Ok(Content::seq(self.elements))
    }

    fn add_element(&mut self, element: i32) {
        self.elements.push(element);
    }
}

#[test]
fn test_end_with_no_elements() {
    let serializer = Serializer::new();
    let result = serializer.end();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().elements.len(), 0);
}

#[test]
fn test_end_with_elements() {
    let mut serializer = Serializer::new();
    serializer.add_element(1);
    serializer.add_element(2);
    let result = serializer.end();
    assert!(result.is_ok());
    let content = result.unwrap();
    assert_eq!(content.elements.len(), 2);
    assert_eq!(content.elements[0], 1);
    assert_eq!(content.elements[1], 2);
}

