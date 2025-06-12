// Answer 0

#[test]
fn test_parse_unicode_escape_with_lone_surrogate() {
    struct MockRead {
        hex_code: u16,
        index: usize,
    }

    impl MockRead {
        fn new() -> Self {
            MockRead { hex_code: 0xD800, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index == 0 {
                self.index += 1;
                Ok(self.hex_code)
            } else {
                Ok(0x10000)  // Return a valid high surrogate
            }
        }

        fn discard(&mut self) {}

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u'))  // Expecting '\u'
        }
        
        fn read(&mut self) -> Result<u8> {
            Ok(b'\\')
        }
    }

    let mut read = MockRead::new();
    let mut scratch = Vec::new();
    let validate = false;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_valid_character_range() {
    struct MockRead {
        hex_code: u16,
        index: usize,
    }

    impl MockRead {
        fn new() -> Self {
            MockRead { hex_code: 0xDBFF, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index == 0 {
                self.index += 1;
                Ok(self.hex_code)
            } else {
                Ok(0x10000)  // Return a valid high surrogate
            }
        }

        fn discard(&mut self) {}

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u'))  // Expecting '\u'
        }
    }

    let mut read = MockRead::new();
    let mut scratch = Vec::new();
    let validate = false;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

#[test]
fn test_parse_unicode_escape_high_surrogate() {
    struct MockRead {
        hex_code: u16,
        index: usize,
    }

    impl MockRead {
        fn new() -> Self {
            MockRead { hex_code: 0xDB80, index: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.index == 0 {
                self.index += 1;
                Ok(self.hex_code)
            } else {
                Ok(0xDFFF)  // Return a valid low surrogate
            }
        }

        fn discard(&mut self) {}

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'u'))  // Expecting '\u'
        }
    }

    let mut read = MockRead::new();
    let mut scratch = Vec::new();
    let validate = false;

    let _ = parse_unicode_escape(&mut read, validate, &mut scratch);
}

