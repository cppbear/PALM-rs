// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct MockDecoder {
        input: Vec<char>,
        index: usize,
    }

    impl MockDecoder {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.index < self.input.len() {
                let value = self.input[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(ErrorCode::EndOfInput.into())
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next_or_eof()?;
            let b = self.next_or_eof()?;
            let c = self.next_or_eof()?;
            let d = self.next_or_eof()?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }
    }

    let mut decoder = MockDecoder {
        input: vec!['1', '0', '0', '1'],
        index: 0,
    };
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Ok(4097));  // 0x1001 in decimal
}

#[test]
#[should_panic]
fn test_decode_hex_escape_invalid_escape() {
    struct MockDecoder {
        input: Vec<char>,
        index: usize,
    }

    impl MockDecoder {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.index < self.input.len() {
                let value = self.input[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(ErrorCode::EndOfInput.into())
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next_or_eof()?;
            let b = self.next_or_eof()?;
            let c = self.next_or_eof()?;
            let d = self.next_or_eof()?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }
    }

    let mut decoder = MockDecoder {
        input: vec!['G', 'H', 'I', 'J'],  // Invalid hex digits
        index: 0,
    };
    let _ = decoder.decode_hex_escape();  // should panic due to invalid escape
}

#[test]
fn test_decode_hex_escape_eof() {
    struct MockDecoder {
        input: Vec<char>,
        index: usize,
    }

    impl MockDecoder {
        fn next_or_eof(&mut self) -> Result<char> {
            if self.index < self.input.len() {
                let value = self.input[self.index];
                self.index += 1;
                Ok(value)
            } else {
                Err(ErrorCode::EndOfInput.into())
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            let a = self.next_or_eof()?;
            let b = self.next_or_eof()?;
            let c = self.next_or_eof()?;
            let d = self.next_or_eof()?;
            match decode_four_hex_digits(a, b, c, d) {
                Some(val) => Ok(val),
                None => error(self, ErrorCode::InvalidEscape),
            }
        }
    }

    let mut decoder = MockDecoder {
        input: vec!['A', 'B'],  // Not enough input
        index: 0,
    };
    let result = decoder.decode_hex_escape();
    assert!(result.is_err());  // Expecting an error due to insufficient input
}

