// Answer 0

fn test_parse_unicode_escape_invalid_leading_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> MockRead {
        fn new(data: &'de [u8]) -> Self {
            Self { data: data.to_vec(), index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, &'static str> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err("End of input")
            }
        }
        
        fn peek_or_eof(&self) -> Result<u8, &'static str> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err("End of input")
            }
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }
    }

    let mut mock_read = MockRead::new(&[0xD800, b'\\', b'u', 0xC2]);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut mock_read, false, &mut scratch);
    assert!(result.is_err());
}

fn test_parse_unicode_escape_invalid_trailing_surrogate() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> MockRead {
        fn new(data: &'de [u8]) -> Self {
            Self { data: data.to_vec(), index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, &'static str> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err("End of input")
            }
        }
        
        fn peek_or_eof(&self) -> Result<u8, &'static str> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err("End of input")
            }
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }
    }

    let mut mock_read = MockRead::new(&[0xD800, b'\\', b'u', 0xD800]);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut mock_read, false, &mut scratch);
    assert!(result.is_ok());
}

fn test_parse_unicode_escape_unexpected_end() {
    struct MockRead {
        data: Vec<u8>,
        index: usize,
    }

    impl<'de> MockRead {
        fn new(data: &'de [u8]) -> Self {
            Self { data: data.to_vec(), index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, &'static str> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val as u32)
            } else {
                Err("End of input")
            }
        }
        
        fn peek_or_eof(&self) -> Result<u8, &'static str> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err("End of input")
            }
        }

        fn discard(&mut self) {
            if self.index < self.data.len() {
                self.index += 1;
            }
        }
    }

    let mut mock_read = MockRead::new(&[0xD800, b'\\']);
    let mut scratch = Vec::new();
    let result = parse_unicode_escape(&mut mock_read, true, &mut scratch);
    assert!(result.is_err());
}

