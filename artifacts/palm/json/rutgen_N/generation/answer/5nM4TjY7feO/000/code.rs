// Answer 0

#[test]
fn test_peek_or_null_with_value() {
    struct MockPeek {
        data: Vec<u8>,
        index: usize,
    }

    impl MockPeek {
        fn new(data: Vec<u8>) -> Self {
            MockPeek { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<&u8>> {
            if self.index < self.data.len() {
                Ok(Some(&self.data[self.index]))
            } else {
                Ok(None)
            }
        }
    }

    let mut mock = MockPeek::new(vec![b'a', b'b', b'c']);
    assert_eq!(mock.peek_or_null().unwrap(), b'a');
}

#[test]
fn test_peek_or_null_with_no_value() {
    struct MockPeek {
        data: Vec<u8>,
        index: usize,
    }

    impl MockPeek {
        fn new(data: Vec<u8>) -> Self {
            MockPeek { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<&u8>> {
            if self.index < self.data.len() {
                Ok(Some(&self.data[self.index]))
            } else {
                Ok(None)
            }
        }
    }

    let mut mock = MockPeek::new(vec![]);
    assert_eq!(mock.peek_or_null().unwrap(), b'\x00');
}

