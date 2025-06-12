// Answer 0

#[test]
fn test_decode_hex_escape_valid() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.index as u64)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.index as u64)
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate decoding a valid hex escape
            if self.data.len() >= 4 {
                let hex = str::from_utf8(&self.data[0..4]).unwrap();
                let value = u16::from_str_radix(hex, 16).map_err(|_| Error::custom("Invalid hex"))?;
                Ok(value)
            } else {
                Err(Error::custom("Not enough data"))
            }
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader {
        data: b"1A2B".to_vec(),
        index: 0,
    };

    let result = reader.decode_hex_escape();
    assert_eq!(result.unwrap(), 0x1A2B);
}

#[test]
#[should_panic(expected = "Invalid hex")]
fn test_decode_hex_escape_invalid() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                Ok(Some(self.data[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.index as u64)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.index as u64)
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            // Simulate decoding an invalid hex escape
            if self.data.len() >= 4 {
                let hex = str::from_utf8(&self.data[0..4]).unwrap();
                u16::from_str_radix(hex, 16).map_err(|_| Error::custom("Invalid hex"))?;
            }
            Err(Error::custom("Invalid hex"))
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader {
        data: b"XYZZ".to_vec(),
        index: 0,
    };

    let _ = reader.decode_hex_escape();
}

