// Answer 0

#[test]
fn test_peek_with_valid_input() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = TestReader::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(reader.peek().unwrap(), Some(1));
}

#[test]
fn test_peek_at_end_of_data() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: data.len() }
        }
        
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = TestReader::new(vec![1, 2, 3, 4, 5]);
    assert_eq!(reader.peek().unwrap(), None);
}

#[should_panic]
#[test]
fn test_peek_with_error_condition() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>, &'static str> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                panic!("Attempting to peek beyond data bounds");
            }
        }
    }

    let mut reader = TestReader::new(vec![]);
    let _ = reader.peek().unwrap(); // This should cause a panic
}

