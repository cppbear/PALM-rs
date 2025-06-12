// Answer 0

#[test]
fn test_peek_with_data_available() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![1, 2, 3]);
    let result = reader.peek();
    assert_eq!(result.ok(), Some(Some(1)));
}

#[test]
fn test_peek_with_no_data() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![]);
    let result = reader.peek();
    assert_eq!(result.ok(), Some(None));
}

#[test]
fn test_peek_with_index_at_end() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, index: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, ()> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![1, 2, 3]);
    reader.index = 3;
    let result = reader.peek();
    assert_eq!(result.ok(), Some(None));
}

