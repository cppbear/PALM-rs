// Answer 0

#[test]
fn test_parse_unicode_escape_valid_bmp() {
    struct MockRead {
        hex_escape_values: Vec<u16>,
        index: usize,
        peek: Option<u8>,
    }

    impl MockRead {
        fn new(hex_escape_values: Vec<u16>, peek: Option<u8>) -> Self {
            MockRead {
                hex_escape_values,
                index: 0,
                peek,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
            if self.index < self.hex_escape_values.len() {
                let val = self.hex_escape_values[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err("EOF")
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&self) -> Result<u8, &'static str> {
            self.peek.map_or(Err("EOF"), |v| Ok(v))
        }
    }

    // Test with surrogate pair starting from 0xD800
    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, 0xDC00], Some(b'\\'));
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![0xF0, 0x9F, 0x98, 0x80]); // Check the output matches the expected UTF-8 byte sequence
}

#[test]
fn test_parse_unicode_escape_valid_lone_surrogate() {
    struct MockRead {
        hex_escape_values: Vec<u16>,
        index: usize,
        peek: Option<u8>,
    }

    impl MockRead {
        fn new(hex_escape_values: Vec<u16>, peek: Option<u8>) -> Self {
            MockRead {
                hex_escape_values,
                index: 0,
                peek,
            }
        }

        fn decode_hex_escape(&mut self) -> Result<u16, &'static str> {
            if self.index < self.hex_escape_values.len() {
                let val = self.hex_escape_values[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err("EOF")
            }
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&self) -> Result<u8, &'static str> {
            self.peek.map_or(Err("EOF"), |v| Ok(v))
        }
    }

    // Test with a lone leading surrogate (should pass because validate is false)
    let mut scratch = Vec::new();
    let mut read = MockRead::new(vec![0xD800, 0xD800], Some(b'\\')); // Lone leading surrogate
    let result = parse_unicode_escape(&mut read, false, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![0xED, 0xA0, 0x80]); // Expected output for the lone surrogate
}

