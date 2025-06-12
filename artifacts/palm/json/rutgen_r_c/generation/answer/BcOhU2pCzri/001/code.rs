// Answer 0

#[test]
fn test_parse_str_raw_empty_scratch() {
    struct DummyReader {}

    impl private::Sealed for DummyReader {}
    
    impl<'de> Read<'de> for DummyReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(0, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(0, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(
            &'s mut self,
            _: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            Err(Error::from(ErrorCode::UnexpectedEndOfInput))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut scratch: Vec<u8> = Vec::new();
    let mut reader = DummyReader {};
    let result: Result<Reference<'_, _>> = reader.parse_str_raw(&mut scratch);
    assert!(result.is_err());
}

#[test]
fn test_parse_str_raw_valid_input() {
    struct DummyReader {
        input: Vec<u8>,
        position: usize,
    }

    impl private::Sealed for DummyReader {}
    
    impl<'de> Read<'de> for DummyReader {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                let byte = self.input[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.input.len() {
                Ok(Some(self.input[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position::new(self.position, 0, 0)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position, 0, 0)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            scratch.extend_from_slice(&self.input);
            Ok(Reference::Borrowed("valid string"))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut scratch: Vec<u8> = Vec::new();
    let input_data = b"valid string".to_vec();
    let mut reader = DummyReader { input: input_data, position: 0 };
    let result: Result<Reference<'_, _>> = reader.parse_str_raw(&mut scratch);
    assert!(result.is_ok());
    if let Ok(Reference::Copied(data)) = result {
        assert_eq!(data, b"valid string");
    }
}

