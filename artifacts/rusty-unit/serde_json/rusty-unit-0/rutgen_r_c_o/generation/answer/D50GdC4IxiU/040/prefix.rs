// Answer 0

#[test]
fn test_parse_unicode_escape_case1() {
    struct MockRead {
        state: Vec<u8>,
        index: usize,
    }

    impl MockRead {
        fn new(state: Vec<u8>) -> Self {
            MockRead { state, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.state.len() {
                let value = self.state[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.state.len() {
                Ok(Some(self.state[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }
    
    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, b'\\', b'u', 0xDC00]);

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_case2() {
    struct MockRead {
        state: Vec<u8>,
        index: usize,
    }

    impl MockRead {
        fn new(state: Vec<u8>) -> Self {
            MockRead { state, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.state.len() {
                let value = self.state[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.state.len() {
                Ok(Some(self.state[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }
    
    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xDBFF, b'\\', b'u', 0xDFFF]);

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_case3() {
    struct MockRead {
        state: Vec<u8>,
        index: usize,
    }

    impl MockRead {
        fn new(state: Vec<u8>) -> Self {
            MockRead { state, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.state.len() {
                let value = self.state[self.index];
                self.index += 1;
                Ok(value as u16)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.index < self.state.len() {
                Ok(Some(self.state[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }
    }
    
    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, b'\\', b'u', 0xDBFF]);

    let _ = parse_unicode_escape(&mut read, false, &mut scratch);
}

