// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct Decoder {
        slice: [u8; 4],
        index: usize,
    }

    impl Decoder {
        fn new(slice: [u8; 4]) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, String> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape".to_string()),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("Eof While Parsing String".to_string())
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        // Simplistic conversion for testing purposes
        let hex = format!("{:01x}{:01x}{:01x}{:01x}", a, b, c, d);
        u16::from_str_radix(&hex, 16).ok()
    }

    let mut decoder = Decoder::new([b'1', b'2', b'3', b'4']);
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Ok(0x1234));
}

#[test]
fn test_decode_hex_escape_eof() {
    struct Decoder {
        slice: [u8; 2],
        index: usize,
    }

    impl Decoder {
        fn new(slice: [u8; 2]) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, String> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape".to_string()),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("Eof While Parsing String".to_string())
                }
            }
        }
    }

    fn decode_four_hex_digits(_: u8, _: u8, _: u8, _: u8) -> Option<u16> {
        None
    }

    let mut decoder = Decoder::new([b'1', b'2']);
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Err("Eof While Parsing String".to_string()));
}

#[test]
fn test_decode_hex_escape_invalid() {
    struct Decoder {
        slice: [u8; 4],
        index: usize,
    }

    impl Decoder {
        fn new(slice: [u8; 4]) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, String> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape".to_string()),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("Eof While Parsing String".to_string())
                }
            }
        }
    }

    fn decode_four_hex_digits(_: u8, _: u8, _: u8, _: u8) -> Option<u16> {
        None
    }

    let mut decoder = Decoder::new([b'Z', b'X', b'Y', b'W']);
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Err("Invalid Escape".to_string()));
}

