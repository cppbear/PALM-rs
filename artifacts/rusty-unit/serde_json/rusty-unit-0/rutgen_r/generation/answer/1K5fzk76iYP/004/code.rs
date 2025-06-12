// Answer 0

fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
    // Dummy implementation to match context
    Some((a as u16) << 12 | (b as u16) << 8 | (c as u16) << 4 | (d as u16))
}

struct Decoder {
    data: Vec<u8>,
    index: usize,
}

impl Decoder {
    fn new(data: Vec<u8>) -> Self {
        Self { data, index: 0 }
    }

    fn next_or_eof(&mut self) -> Result<u8, &'static str> {
        if self.index < self.data.len() {
            let val = self.data[self.index];
            self.index += 1;
            Ok(val)
        } else {
            Err("EOF reached")
        }
    }

    fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
        let a = self.next_or_eof()?;
        let b = self.next_or_eof()?;
        let c = self.next_or_eof()?;
        let d = self.next_or_eof()?;
        match decode_four_hex_digits(a, b, c, d) {
            Some(val) => Ok(val),
            None => Err("Invalid Escape"),
        }
    }
}

#[test]
fn test_decode_hex_escape_success() {
    let mut decoder = Decoder::new(vec![0x1A, 0x2B, 0x3C, 0x4D]);
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Ok(0x1A2B3C4D));
}

#[test]
fn test_decode_hex_escape_eof_before_end() {
    let mut decoder = Decoder::new(vec![0x1A, 0x2B]);
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Err("EOF reached"));
}

#[test]
fn test_decode_hex_escape_invalid_escape() {
    let mut decoder = Decoder::new(vec![0x1A, 0x2B, 0x3C, 0x00]); // Assuming decode_four_hex_digits would return None for 0
    let result = decoder.decode_hex_escape();
    assert_eq!(result, Err("Invalid Escape"));
}

