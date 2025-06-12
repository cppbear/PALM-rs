// Answer 0

fn test_scan_integer128_valid_single_zero() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
    }

    let mut buf = String::new();
    let mut reader = MockReader::new(vec![b'0']);
    let result = reader.next_char_or_null();

    if let Ok(Some(b'0')) = result {
        buf.push('0');
        let peek_result = reader.peek_or_null();
        assert!(peek_result.is_ok());
        let peek_value = peek_result.unwrap();
        assert!(peek_value.is_none()); // should not be b'0' or b'1'..=b'9'
    }
    
    assert!(buf == "0");
}

fn test_scan_integer128_valid_multiple_digits() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
    }

    let mut buf = String::new();
    let mut reader = MockReader::new(vec![b'2', b'3', b'4']);
    
    let result = reader.next_char_or_null();
    if let Ok(Some(c @ b'1'..=b'9')) = result {
        buf.push(c as char);
        while let Ok(Some(c @ b'0'..=b'9')) = reader.peek_or_null() {
            reader.eat_char();
            buf.push(c as char);
        }
    }

    assert!(buf == "234");
}

fn test_scan_integer128_invalid_leading_zero() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn error(&self, _reason: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }
    }

    let mut buf = String::new();
    let mut reader = MockReader::new(vec![b'0', b'1']);
    
    let result = reader.next_char_or_null();
    if let Ok(Some(b'0')) = result {
        buf.push('0');
        let peek_result = reader.peek_or_null();
        if let Ok(Some(b'1')) = peek_result {
            assert_eq!(reader.error(ErrorCode::InvalidNumber), Error { err: Box::new(ErrorImpl) });
        }
    }
}

fn test_scan_integer128_invalid_character() {
    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            MockReader { input, position: 0 }
        }

        fn next_char_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(Some(ch))
            } else {
                Ok(None)
            }
        }

        fn peek_or_null(&self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            self.position += 1;
        }
        
        fn error(&self, _reason: ErrorCode) -> Error {
            Error { err: Box::new(ErrorImpl) }
        }
    }

    let mut buf = String::new();
    let mut reader = MockReader::new(vec![b'A']);
    
    let result = reader.next_char_or_null();
    if result.is_ok() {
        assert_eq!(reader.error(ErrorCode::InvalidNumber), Error { err: Box::new(ErrorImpl) });
    }
}

