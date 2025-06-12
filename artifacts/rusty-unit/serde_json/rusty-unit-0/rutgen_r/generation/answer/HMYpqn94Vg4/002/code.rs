// Answer 0

#[test]
fn test_decode_hex_escape_valid_input() {
    struct TestDecoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestDecoder {
        fn new(slice: Vec<u8>) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, ()> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err(()), // Simulating error for None case
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err(()) // Simulating EOF error
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() && c.is_ascii_hexdigit() && d.is_ascii_hexdigit() {
            Some(
                ((a.to_digit(16).unwrap() as u16) << 12)
                | ((b.to_digit(16).unwrap() as u16) << 8)
                | ((c.to_digit(16).unwrap() as u16) << 4)
                | (d.to_digit(16).unwrap() as u16)
            )
        } else {
            None
        }
    }

    let mut decoder = TestDecoder::new(vec![b'1', b'2', b'3', b'4']);
    assert_eq!(decoder.decode_hex_escape(), Ok(0x1234));
}

#[test]
fn test_decode_hex_escape_invalid_hex() {
    struct TestDecoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestDecoder {
        fn new(slice: Vec<u8>) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, ()> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err(()), // Simulating error for None case
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err(()) // Simulating EOF error
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() && c.is_ascii_hexdigit() && d.is_ascii_hexdigit() {
            Some(
                ((a.to_digit(16).unwrap() as u16) << 12)
                | ((b.to_digit(16).unwrap() as u16) << 8)
                | ((c.to_digit(16).unwrap() as u16) << 4)
                | (d.to_digit(16).unwrap() as u16)
            )
        } else {
            None
        }
    }

    let mut decoder = TestDecoder::new(vec![b'g', b'h', b'i', b'j']); // Invalid hex digits
    assert_eq!(decoder.decode_hex_escape(), Err(()));
}

#[test]
fn test_decode_hex_escape_eof() {
    struct TestDecoder {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestDecoder {
        fn new(slice: Vec<u8>) -> Self {
            Self { slice, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, ()> {
            match &self.slice[self.index..] {
                [a, b, c, d, ..] => {
                    self.index += 4;
                    match decode_four_hex_digits(*a, *b, *c, *d) {
                        Some(val) => Ok(val),
                        None => Err(()), // Simulating error for None case
                    }
                }
                _ => {
                    self.index = self.slice.len();
                    Err(()) // Simulating EOF error
                }
            }
        }
    }

    fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
        if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() && c.is_ascii_hexdigit() && d.is_ascii_hexdigit() {
            Some(
                ((a.to_digit(16).unwrap() as u16) << 12)
                | ((b.to_digit(16).unwrap() as u16) << 8)
                | ((c.to_digit(16).unwrap() as u16) << 4)
                | (d.to_digit(16).unwrap() as u16)
            )
        } else {
            None
        }
    }

    let mut decoder = TestDecoder::new(vec![]);
    assert_eq!(decoder.decode_hex_escape(), Err(())); // Expect EOF error
}

