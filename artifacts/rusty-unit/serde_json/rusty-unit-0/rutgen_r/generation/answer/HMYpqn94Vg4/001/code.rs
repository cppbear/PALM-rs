// Answer 0

#[test]
fn test_decode_hex_escape_success() {
    struct Decoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl Decoder {
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

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        // Simulating successful decoding
        Some((a as u16) << 12 | (b as u16) << 8 | (c as u16) << 4 | (d as u16))
    }

    fn error(_decoder: &mut Decoder, _code: ErrorCode) -> Result<u16> {
        // Simulating an error return
        Err(Error::new(ErrorCode::InvalidEscape))
    }

    #[derive(Debug)]
    enum ErrorCode {
        InvalidEscape,
        EofWhileParsingString,
    }

    let mut decoder = Decoder {
        slice: vec![b'1', b'2', b'3', b'4', b'5'],
        index: 0,
    };
    
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Ok(0x1234)); // 0x1234 is the expected value for '1', '2', '3', '4'
}

#[test]
fn test_decode_hex_escape_eof() {
    struct Decoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl Decoder {
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

    fn decode_four_hex_digits(_a: u8, _b: u8, _c: u8, _d: u8) -> Option<u16> {
        None
    }

    fn error(_decoder: &mut Decoder, _code: ErrorCode) -> Result<u16> {
        Err(Error::new(ErrorCode::EofWhileParsingString))
    }

    #[derive(Debug)]
    enum ErrorCode {
        InvalidEscape,
        EofWhileParsingString,
    }

    let mut decoder = Decoder {
        slice: vec![b'1', b'2', b'3'], // Not enough bytes
        index: 0,
    };

    let result = decoder.decode_hex_escape();
    assert!(result.is_err());
}

