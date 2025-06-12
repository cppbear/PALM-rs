// Answer 0

#[test]
fn test_parse_unicode_escape_valid_boundary_conditions() {
    use core::marker::PhantomData;

    struct MockRead {
        data: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn peek(&mut self) -> Option<u8> {
            self.data.get(self.position).copied()
        }

        fn discard(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16, Error> {
            if self.position + 4 <= self.data.len() {
                let hex_str = &self.data[self.position..self.position + 4];
                self.position += 4;
                let hex_value = str::from_utf8(hex_str)
                    .map_err(|_| Error::new(ErrorCode::InvalidEscape))?
                    .parse::<u16>()
                    .map_err(|_| Error::new(ErrorCode::InvalidNumber))?;
                Ok(hex_value.into())
            } else {
                Err(Error::new(ErrorCode::EofWhileParsingString))
            }
        }
    }

    impl<'de> Read<'de> for MockRead {}

    let mut scratch: Vec<u8> = Vec::new();
    let mut read = MockRead::new(vec![b'\\', b'u', b'D', b'8', b'0', b'0', b'0', b'0', b'\\', b'u', b'D', b'C', b'0', b'0', b'0', b'0']);
    
    assert!(parse_unicode_escape(&mut read, false, &mut scratch).is_ok());
    assert_eq!(scratch.len(), 4); // Two surrogates should produce a single codepoint
}

