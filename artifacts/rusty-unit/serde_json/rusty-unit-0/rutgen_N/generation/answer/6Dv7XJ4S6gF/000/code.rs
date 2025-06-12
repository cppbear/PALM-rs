// Answer 0

#[derive(Default)]
struct MockDelegate {
    data: Vec<u8>,
    index: usize,
}

impl MockDelegate {
    fn new(data: Vec<u8>) -> Self {
        MockDelegate { data, index: 0 }
    }

    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        if self.index < self.data.len() {
            Ok(Some(self.data[self.index]))
        } else {
            Ok(None)
        }
    }
}

struct PeekableReader<'a> {
    delegate: &'a mut MockDelegate,
}

impl<'a> PeekableReader<'a> {
    fn peek(&mut self) -> Result<Option<u8>, &'static str> {
        self.delegate.peek()
    }
}

#[test]
fn test_peek_returns_some_value() {
    let mut delegate = MockDelegate::new(vec![1, 2, 3]);
    let mut reader = PeekableReader { delegate: &mut delegate };
    let result = reader.peek().unwrap();
    assert_eq!(result, Some(1));
}

#[test]
fn test_peek_returns_none_when_empty() {
    let mut delegate = MockDelegate::new(vec![]);
    let mut reader = PeekableReader { delegate: &mut delegate };
    let result = reader.peek().unwrap();
    assert_eq!(result, None);
}

#[test]
fn test_peek_returns_some_after_value_access() {
    let mut delegate = MockDelegate::new(vec![1, 2, 3]);
    let mut reader = PeekableReader { delegate: &mut delegate };
    // Call peek once
    let result = reader.peek().unwrap();
    assert_eq!(result, Some(1));
    // Call again, should still return the same value
    let result = reader.peek().unwrap();
    assert_eq!(result, Some(1));
}

