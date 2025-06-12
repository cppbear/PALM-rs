// Answer 0

#[derive(Default)]
struct MockReader {
    data: Vec<u8>,
    position: usize,
}

impl MockReader {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }
}

impl MockReader {
    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        if self.position < self.data.len() {
            Ok(Some(self.data[self.position]))
        } else {
            Ok(None)
        }
    }
}

#[test]
fn test_peek_some() {
    let mut reader = MockReader::new(vec![1, 2, 3]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), Some(1));
}

#[test]
fn test_peek_none() {
    let mut reader = MockReader::new(vec![]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_peek_after_consume() {
    let mut reader = MockReader::new(vec![1, 2, 3]);
    let _ = reader.peek(); // Call peek first
    reader.position += 1; // Simulate consume
    let result = reader.peek();
    assert_eq!(result.unwrap(), Some(2));
}

#[test]
fn test_peek_at_end() {
    let mut reader = MockReader::new(vec![1, 2, 3]);
    reader.position = 3; // Move to the end
    let result = reader.peek();
    assert_eq!(result.unwrap(), None);
}

