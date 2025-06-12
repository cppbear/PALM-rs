// Answer 0

#[test]
fn test_parse_unicode_escape_with_leading_surrogate() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position >= self.input.len() {
                return Err(Error::from(ErrorCode::EofWhileParsingValue));
            }
            let val = self.input[self.position];
            self.position += 1;
            Ok(val as u16)  // Simulate the decode
        }

        fn peek(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    let mut reader = TestReader::new(vec![0xD800, b'\\', b'u', 0xD800]);
    let mut scratch = Vec::new();
    let validate = false;

    parse_unicode_escape(&mut reader, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_with_trailing_surrogate() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position >= self.input.len() {
                return Err(Error::from(ErrorCode::EofWhileParsingValue));
            }
            let val = self.input[self.position];
            self.position += 1;
            Ok(val as u16)  // Simulate the decode
        }

        fn peek(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    let mut reader = TestReader::new(vec![0xD800, b'\\', b'u', 0xDC00]);
    let mut scratch = Vec::new();
    let validate = true;

    parse_unicode_escape(&mut reader, validate, &mut scratch);
} 

#[test]
fn test_parse_unicode_escape_non_surrogate() {
    struct TestReader {
        input: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.position >= self.input.len() {
                return Err(Error::from(ErrorCode::EofWhileParsingValue));
            }
            let val = self.input[self.position];
            self.position += 1;
            Ok(val as u16)  // Simulate the decode
        }

        fn peek(&mut self) -> Option<u8> {
            if self.position < self.input.len() {
                Some(self.input[self.position])
            } else {
                None
            }
        }

        fn discard(&mut self) {
            self.position += 1;
        }
    }

    let mut reader = TestReader::new(vec![0xD800, b'\\', b'u', 0xDC00, b'\\', b'c']);
    let mut scratch = Vec::new();
    let validate = false;

    parse_unicode_escape(&mut reader, validate, &mut scratch);
}

