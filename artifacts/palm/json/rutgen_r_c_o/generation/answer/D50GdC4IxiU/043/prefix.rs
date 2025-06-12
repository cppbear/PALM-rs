// Answer 0

#[test]
fn test_parse_unicode_escape_leading_surrogate() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos >= self.data.len() {
                return error(&mut self, ErrorCode::EofWhileParsingString);
            }
            let val = self.data[self.pos];
            self.pos += 1;
            Ok(val as u16)
        }

        fn peek(&self) -> Option<u8> {
            if self.pos < self.data.len() {
                Some(self.data[self.pos])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
        
        fn read(&mut self) -> Option<u8> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    impl Read<'static> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.decode_hex_escape()
        }

        fn peek(&mut self) -> Result<u8> {
            self.peek().ok_or_else(|| error(self, ErrorCode::EofWhileParsingString))
        }

        fn discard(&mut self) {
            self.discard();
        }
        
        fn next(&mut self) -> Option<u8> {
            self.read()
        }
    }

    let mut mock_reader = MockReader::new(vec![0xD800, b'\\']);
    let mut scratch = Vec::new();
    let validate = true;
    let result = parse_unicode_escape(&mut mock_reader, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_trailing_surrogate() {
    struct MockReader {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, pos: 0 }
        }
        
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos >= self.data.len() {
                return error(&mut self, ErrorCode::EofWhileParsingString);
            }
            let val = self.data[self.pos];
            self.pos += 1;
            Ok(val as u16)
        }

        fn peek(&self) -> Option<u8> {
            if self.pos < self.data.len() {
                Some(self.data[self.pos])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
        
        fn read(&mut self) -> Option<u8> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    impl Read<'static> for MockReader {
        fn decode_hex_escape(&mut self) -> Result<u16> {
            self.decode_hex_escape()
        }

        fn peek(&mut self) -> Result<u8> {
            self.peek().ok_or_else(|| error(self, ErrorCode::EofWhileParsingString))
        }

        fn discard(&mut self) {
            self.discard();
        }
        
        fn next(&mut self) -> Option<u8> {
            self.read()
        }
    }

    let mut mock_reader = MockReader::new(vec![0xDBFF, b'\\']);
    let mut scratch = Vec::new();
    let validate = true;
    let result = parse_unicode_escape(&mut mock_reader, validate, &mut scratch);
}

