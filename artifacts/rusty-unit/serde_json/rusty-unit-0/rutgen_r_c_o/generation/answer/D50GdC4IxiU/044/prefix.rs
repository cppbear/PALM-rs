// Answer 0

#[test]
fn test_parse_unicode_escape_leading_surrogate() {
    struct MockRead {
        hex_values: Vec<u16>,
        index: usize,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>) -> Self {
            Self { hex_values, index: 0 }
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
        fn peek(&self) -> Result<Option<u8>> {
            Ok(Some(b'\\'))
        }
        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, 0xDC00]);
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert!(result.is_ok());
}

#[test]
fn test_parse_unicode_escape_trailing_surrogate() {
    struct MockRead {
        hex_values: Vec<u16>,
        index: usize,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>) -> Self {
            Self { hex_values, index: 0 }
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
        fn peek(&self) -> Result<Option<u8>> {
            Ok(Some(b'\\'))
        }
        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xDBFF, 0xD800]);
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert!(result.is_ok());
} 

#[test]
fn test_parse_unicode_escape_both_surrogates() {
    struct MockRead {
        hex_values: Vec<u16>,
        index: usize,
    }

    impl MockRead {
        fn new(hex_values: Vec<u16>) -> Self {
            Self { hex_values, index: 0 }
        }
        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index < self.hex_values.len() {
                let value = self.hex_values[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingValue))
            }
        }
        fn peek(&self) -> Result<Option<u8>> {
            Ok(Some(b'\\'))
        }
        fn discard(&mut self) {}
    }

    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, 0xDBFF]);
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert!(result.is_ok());
}

