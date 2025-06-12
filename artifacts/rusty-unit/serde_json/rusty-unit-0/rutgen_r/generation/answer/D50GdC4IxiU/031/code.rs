// Answer 0

#[test]
fn test_parse_unicode_escape_valid_character() {
    use std::io::{self, Read};

    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, io::Error> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                return Ok(ch as u32);
            }
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"))
        }

        fn discard(&mut self) {
            // Discard logic can be a no-op in mock
        }

        fn peek_or_eof(&mut self) -> Result<u8, io::Error> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"))
            }
        }
    }

    let mut reader = MockReader::new(vec![0x61]); // 'a' in Unicode
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_unicode_escape(&mut reader, validate, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0x61]); // Ensure 'a' has been appended correctly
}

#[test]
fn test_parse_unicode_escape_with_leading_surrogate() {
    use std::io::{self, Read};

    struct MockReader {
        input: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u32, io::Error> {
            if self.position < self.input.len() {
                let ch = self.input[self.position];
                self.position += 1;
                return Ok(ch as u32);
            }
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"))
        }

        fn discard(&mut self) {
            // Discard logic can be a no-op in mock
        }

        fn peek_or_eof(&mut self) -> Result<u8, io::Error> {
            if self.position < self.input.len() {
                Ok(self.input[self.position])
            } else {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF reached"))
            }
        }
    }

    let mut reader = MockReader::new(vec![0xD8, 0x00]); // Leading surrogate
    let mut scratch = Vec::new();
    let validate = false;

    let result = parse_unicode_escape(&mut reader, validate, &mut scratch);
    
    assert!(result.is_ok());
    assert_eq!(scratch, vec![0xD8, 0x00]); // Ensure leading surrogate is processed
}

