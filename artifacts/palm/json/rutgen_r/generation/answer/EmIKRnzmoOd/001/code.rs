// Answer 0

fn test_scan_integer128_err_on_next_char_or_null() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
        
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Err(()) // Simulate an error when no more characters
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(()) // Simulate an error when no more characters
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ()> {
            match self.next_char_or_null() {
                Ok(b'0') => {
                    buf.push('0');
                    match self.peek_or_null() {
                        Ok(c @ b'0'..=b'9') => Err(()), // Invalid number
                        _ => Ok(()),
                    }
                }
                Ok(c @ b'1'..=b'9') => {
                    buf.push(c as char);
                    while let Ok(c @ b'0'..=b'9') = self.peek_or_null() {
                        self.eat_char();
                        buf.push(c as char);
                    }
                    Ok(())
                }
                _ => Err(()), // Invalid number
            }
        }
    }

    let mut reader = TestReader::new(vec![]);
    let mut buf = String::new();
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_err());
}

fn test_scan_integer128_err_on_peek_or_null() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
        
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Err(()) // Simulate an error when no more characters
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(()) // Simulate an error when no more characters
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ()> {
            match self.next_char_or_null() {
                Ok(b'0') => {
                    buf.push('0');
                    match self.peek_or_null() {
                        Ok(c @ b'0'..=b'9') => Err(()), // Invalid number
                        _ => Ok(()),
                    }
                }
                Ok(c @ b'1'..=b'9') => {
                    buf.push(c as char);
                    while let Ok(c @ b'0'..=b'9') = self.peek_or_null() {
                        self.eat_char();
                        buf.push(c as char);
                    }
                    Ok(())
                }
                _ => Err(()), // Invalid number
            }
        }
    }

    let mut reader = TestReader::new(vec![b'1']);
    let mut buf = String::new();
    let _ = reader.scan_integer128(&mut buf); // Consume '1'
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_err());
}

fn test_scan_integer128_invalid_number() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
        
        fn next_char_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                Ok(ch)
            } else {
                Err(()) // Simulate an error when no more characters
            }
        }

        fn peek_or_null(&mut self) -> Result<u8, ()> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(()) // Simulate an error when no more characters
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }
        
        fn scan_integer128(&mut self, buf: &mut String) -> Result<(), ()> {
            match self.next_char_or_null() {
                Ok(b'0') => {
                    buf.push('0');
                    match self.peek_or_null() {
                        Ok(c @ b'0'..=b'9') => Err(()), // Invalid number
                        _ => Ok(()),
                    }
                }
                Ok(c @ b'1'..=b'9') => {
                    buf.push(c as char);
                    while let Ok(c @ b'0'..=b'9') = self.peek_or_null() {
                        self.eat_char();
                        buf.push(c as char);
                    }
                    Ok(())
                }
                _ => Err(()), // Invalid number
            }
        }
    }

    let mut reader = TestReader::new(vec![b'0', b'1']); // '0' followed by '1' should be invalid
    let mut buf = String::new();
    let result = reader.scan_integer128(&mut buf);
    assert!(result.is_err());
}

