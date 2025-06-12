// Answer 0

fn test_peek_or_eof_success() {
    struct MockReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: &[u8]) -> Self {
            Self {
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
    }

    impl Deref for MockReader {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.buffer
        }
    }

    let mut reader = MockReader::new(b"test");
    let result = peek_or_eof(&mut reader);
    assert_eq!(result, Ok(b't'));
}

fn test_peek_or_eof_eof_error() {
    struct MockReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: &[u8]) -> Self {
            Self {
                buffer: data.to_vec(),
                position: data.len(),
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }
        
        fn position(&self) -> (usize, usize) {
            (0, self.position)
        }
    }

    impl Deref for MockReader {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.buffer
        }
    }

    let mut reader = MockReader::new(b"");
    let result = peek_or_eof(&mut reader);
    assert_eq!(result.is_err(), true);
}

fn test_peek_or_eof_with_non_empty_buffer() {
    struct MockReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: &[u8]) -> Self {
            Self {
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
        
        fn position(&self) -> (usize, usize) {
            (0, self.position)
        }
    }

    impl Deref for MockReader {
        type Target = [u8];

        fn deref(&self) -> &Self::Target {
            &self.buffer
        }
    }

    let mut reader = MockReader::new(b"abc");
    reader.position = 1; // Simulating that we're at the second character
    let result = peek_or_eof(&mut reader);
    assert_eq!(result, Ok(b'b'));
}

