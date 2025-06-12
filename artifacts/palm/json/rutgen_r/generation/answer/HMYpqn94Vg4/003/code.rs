// Answer 0

#[test]
fn test_decode_hex_escape_eof_while_parsing_string() {
    struct TestStruct {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn new(slice: Vec<u8>) -> Self {
            TestStruct { index: 0, slice }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            match self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(a, b, c, d) {
                        Some(val) => Ok(val),
                        None => error(self, ErrorCode::InvalidEscape),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    error(self, ErrorCode::EofWhileParsingString)
                }
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![b'\\', b'x']); // Incomplete hex escape
    let result = test_struct.decode_hex_escape();
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.code(), ErrorCode::EofWhileParsingString);
    }
}

#[test]
fn test_decode_hex_escape_incomplete_hex_digits() {
    struct TestStruct {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn new(slice: Vec<u8>) -> Self {
            TestStruct { index: 0, slice }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            match self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(a, b, c, d) {
                        Some(val) => Ok(val),
                        None => error(self, ErrorCode::InvalidEscape),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    error(self, ErrorCode::EofWhileParsingString)
                }
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![b'\\', b'x', b'G']); // Only 3 hex digits
    let result = test_struct.decode_hex_escape();
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.code(), ErrorCode::EofWhileParsingString);
    }
}

