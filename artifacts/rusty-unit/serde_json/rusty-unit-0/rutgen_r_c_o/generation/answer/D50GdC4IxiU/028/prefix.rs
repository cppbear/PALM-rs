// Answer 0

#[test]
fn test_parse_unicode_escape_valid_surrogate_pair() {
    struct MockRead {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(hex: &str) -> Self {
            let buffer = hex.as_bytes().to_vec();
            MockRead { buffer, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.buffer.len() {
                let hex_str = &self.buffer[self.pos..self.pos + 4];
                self.pos += 4;
                u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16)
                    .map_err(|_| Error::from(ErrorCode::InvalidEscape))
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.buffer.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"D800DCAA\\uD801".as_ref());

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_invalid_leading_surrogate() {
    struct MockRead {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(hex: &str) -> Self {
            let buffer = hex.as_bytes().to_vec();
            MockRead { buffer, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.buffer.len() {
                let hex_str = &self.buffer[self.pos..self.pos + 4];
                self.pos += 4;
                u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16)
                    .map_err(|_| Error::from(ErrorCode::InvalidEscape))
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.buffer.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"D800DC00\\uD800".as_ref());

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid_lonely_surrogate() {
    struct MockRead {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(hex: &str) -> Self {
            let buffer = hex.as_bytes().to_vec();
            MockRead { buffer, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.buffer.len() {
                let hex_str = &self.buffer[self.pos..self.pos + 4];
                self.pos += 4;
                u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16)
                    .map_err(|_| Error::from(ErrorCode::InvalidEscape))
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.buffer.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"D800\\uE000".as_ref());

    let result = parse_unicode_escape(&mut read, false, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_invalid_trailing_surrogate() {
    struct MockRead {
        buffer: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(hex: &str) -> Self {
            let buffer = hex.as_bytes().to_vec();
            MockRead { buffer, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, Error> {
            if self.pos < self.buffer.len() {
                let hex_str = &self.buffer[self.pos..self.pos + 4];
                self.pos += 4;
                u16::from_str_radix(std::str::from_utf8(hex_str).unwrap(), 16)
                    .map_err(|_| Error::from(ErrorCode::InvalidEscape))
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }

        fn peek(&self) -> Option<u8> {
            self.buffer.get(self.pos).copied()
        }

        fn discard(&mut self) {
            self.pos += 1;
        }
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(b"D800\\uDC00D800".as_ref());

    let result = parse_unicode_escape(&mut read, true, &mut scratch);
}

