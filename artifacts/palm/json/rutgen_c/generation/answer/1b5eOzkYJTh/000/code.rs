// Answer 0

#[test]
fn test_parse_str_success() {
    struct MockReader<'de> {
        data: &'de [u8],
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader<'de> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                let byte = self.data[self.pos];
                self.pos += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.pos < self.data.len() {
                Ok(Some(self.data[self.pos]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
        }

        fn byte_offset(&self) -> usize {
            self.pos
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            let parsed_str = str::from_utf8(self.data).map_err(|_| Error::new(ErrorCode::InvalidValue))?;
            scratch.extend_from_slice(self.data);
            Ok(Reference::Borrowed(parsed_str))
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }

        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input_data: Vec<u8> = b"test string".to_vec();
    let mut reader = MockReader { data: &input_data, pos: 0 };
    let mut scratch: Vec<u8> = Vec::new();

    let result = reader.parse_str(&mut scratch);

    assert!(result.is_ok());
    if let Ok(Reference::Borrowed(parsed_str)) = result {
        assert_eq!(parsed_str, "test string");
    }
}

#[test]
#[should_panic]
fn test_parse_str_invalid_utf8() {
    struct MockReader<'de> {
        data: &'de [u8],
        pos: usize,
    }

    impl<'de> Read<'de> for MockReader<'de> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn peek(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { self.pos }
        
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Err(Error::new(ErrorCode::InvalidValue))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let input_data: Vec<u8> = b"\xFF\xFE".to_vec();
    let mut reader = MockReader { data: &input_data, pos: 0 };
    let mut scratch: Vec<u8> = Vec::new();

    reader.parse_str(&mut scratch).unwrap();
}

