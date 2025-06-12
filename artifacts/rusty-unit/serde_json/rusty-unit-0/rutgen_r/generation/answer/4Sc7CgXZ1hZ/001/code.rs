// Answer 0

#[test]
fn test_peek_with_no_data() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), None);
}

#[test]
fn test_peek_with_one_byte() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![42]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), Some(42));
}

#[test]
fn test_peek_with_multiple_bytes() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader::new(vec![1, 2, 3, 4]);
    let result = reader.peek();
    assert_eq!(result.unwrap(), Some(1));
} 

#[test]
fn test_peek_after_reading_past_end() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }
    
    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn read(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }
    }

    let mut reader = MockReader::new(vec![5, 6, 7]);
    reader.read(); // reading first byte
    reader.read(); // reading second byte
    reader.read(); // reading third byte
    reader.read(); // attempting to read past the end
    let result = reader.peek();
    assert_eq!(result.unwrap(), None);
}

