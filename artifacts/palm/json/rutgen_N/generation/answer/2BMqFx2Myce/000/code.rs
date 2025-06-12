// Answer 0

#[test]
fn test_parse_whitespace_non_whitespace_byte() {
    struct TestReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: &[u8]) -> Self {
            TestReader {
                buffer: data.to_vec(),
                position: 0,
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }
    }
    
    let mut reader = TestReader::new(b"   a");
    let result = reader.parse_whitespace();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(b'a'));
}

#[test]
fn test_parse_whitespace_eof() {
    struct TestReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: &[u8]) -> Self {
            TestReader {
                buffer: data.to_vec(),
                position: 0,
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.buffer.len() {
                self.position += 1;
            }
        }
    }

    let mut reader = TestReader::new(b"   \n   ");
    let result = reader.parse_whitespace();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), None);
}

