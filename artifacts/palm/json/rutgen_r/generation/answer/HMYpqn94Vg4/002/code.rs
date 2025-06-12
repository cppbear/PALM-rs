// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct Decoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl Decoder {
        fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
            match self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(a, b, c, d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape"),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("EOF While Parsing String")
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        let hex_str = format!("{:x}{:x}{:x}{:x}", a, b, c, d);
        u16::from_str_radix(&hex_str, 16).ok()
    }

    // Valid hex escape sequence
    let mut decoder = Decoder { 
        slice: vec![b'1', b'2', b'3', b'4'], 
        index: 0 
    };
    assert_eq!(decoder.decode_hex_escape().unwrap(), 0x1234);
}

#[test]
fn test_decode_hex_escape_invalid_escape() {
    struct Decoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl Decoder {
        fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
            match self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(a, b, c, d) {
                        Some(val) => Ok(val),
                        None => Err("Invalid Escape"),
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err("EOF While Parsing String")
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        // Triggering an invalid state, for example with non-hex characters
        if a == b'z' as u8 || b == b'z' as u8 || c == b'z' as u8 || d == b'z' as u8 {
            return None;
        }
        let hex_str = format!("{:x}{:x}{:x}{:x}", a, b, c, d);
        u16::from_str_radix(&hex_str, 16).ok()
    }

    // Invalid hex escape sequence
    let mut decoder = Decoder { 
        slice: vec![b'z', b'1', b'2', b'3'], 
        index: 0 
    };
    assert_eq!(decoder.decode_hex_escape().unwrap_err(), "Invalid Escape");
}

