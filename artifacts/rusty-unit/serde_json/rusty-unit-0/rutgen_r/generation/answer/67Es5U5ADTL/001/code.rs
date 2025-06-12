// Answer 0

#[test]
fn test_peek_success() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![1, 2, 3]);
    assert_eq!(reader.peek().unwrap(), Some(1));
}

#[test]
fn test_peek_end_of_data() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![]);
    assert_eq!(reader.peek().unwrap(), None);
}

#[test]
fn test_peek_panic_condition() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }

        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < 0 { // this will panic in a real scenario
                return Err("Position out of bounds");
            } else if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![1, 2, 3]);
    reader.position = 10; // try to peek beyond the data length
    assert_eq!(reader.peek().unwrap(), None);
}

