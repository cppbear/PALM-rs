// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct Decoder {
        index: usize,
        slice: Vec<u8>,
    }

    impl Decoder {
        fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape"),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("Eof While Parsing String")
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        // Assuming this function converts hex digits to a number correctly.
        u16::from_str_radix(&format!("{}{}{}{}", a as char, b as char, c as char, d as char), 16).ok()
    }

    let mut decoder = Decoder {
        index: 0,
        slice: b"abcd".to_vec(),
    };

    let result = decoder.decode_hex_escape();
    assert_eq!(result, Ok(43981)); // 0xABCD
}

#[test]
fn test_decode_hex_escape_eof() {
    struct Decoder {
        index: usize,
        slice: Vec<u8>,
    }

    impl Decoder {
        fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape"),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("Eof While Parsing String")
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        u16::from_str_radix(&format!("{}{}{}{}", a as char, b as char, c as char, d as char), 16).ok()
    }

    let mut decoder = Decoder {
        index: 0,
        slice: b"abc".to_vec(), // Not enough data for a hex escape
    };

    let result = decoder.decode_hex_escape();
    assert_eq!(result, Err("Eof While Parsing String"));
}

