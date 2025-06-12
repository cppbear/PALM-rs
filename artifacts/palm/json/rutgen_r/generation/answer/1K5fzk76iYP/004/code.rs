// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct TestReader {
        chars: Vec<char>,
        index: usize,
    }

    impl TestReader {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<char> {
            if self.index < self.chars.len() {
                let val = self.chars[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(ErrorCode::EndOfInput)
            }
        }
    }

    let mut reader = TestReader::new(vec!['1', 'a', 'f', 'f']); // Valid hex escape
    let result = decode_hex_escape(&mut reader);
    assert_eq!(result, Ok(0x1af));
}

#[test]
#[should_panic]
fn test_decode_hex_escape_invalid_escape() {
    struct TestReader {
        chars: Vec<char>,
        index: usize,
    }

    impl TestReader {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<char> {
            if self.index < self.chars.len() {
                let val = self.chars[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(ErrorCode::EndOfInput)
            }
        }
    }

    let mut reader = TestReader::new(vec!['1', 'g', 'f', 'f']); // Invalid hex escape
    let result = decode_hex_escape(&mut reader);
    assert_eq!(result, Err(ErrorCode::InvalidEscape)); // This line should trigger panic as we expect an Err.
}

#[test]
fn test_decode_hex_escape_eof_before_complete() {
    struct TestReader {
        chars: Vec<char>,
        index: usize,
    }

    impl TestReader {
        fn new(chars: Vec<char>) -> Self {
            Self { chars, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<char> {
            if self.index < self.chars.len() {
                let val = self.chars[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(ErrorCode::EndOfInput)
            }
        }
    }

    let mut reader = TestReader::new(vec!['1', 'a']); // Not enough digits
    let result = decode_hex_escape(&mut reader);
    assert_eq!(result, Err(ErrorCode::EndOfInput)); // Expecting an error due to not enough characters being available.
}

