// Answer 0

#[test]
fn test_parse_unicode_escape_valid_range() {
    struct MockRead {
        data: Vec<u8>,
        pos: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            MockRead { data, pos: 0 }
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            if self.pos < self.data.len() {
                let val = self.data[self.pos];
                self.pos += 1;
                Ok(u16::from(val))
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }

        fn discard(&mut self) {
            if self.pos < self.data.len() {
                self.pos += 1;
            }
        }

        fn peek(&self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn tri<T>(&self, result: Result<T>) -> T {
            result.unwrap()
        }
    }

    impl Read<'_> for MockRead {}

    let mut scratch = Vec::new();
    let mut reader = MockRead::new(vec![0xD8, 0x00, b'\\', b'u', 0xD8, 0x00]);
    
    let result = parse_unicode_escape(&mut reader, false, &mut scratch);
    
    assert!(result.is_ok());
}

