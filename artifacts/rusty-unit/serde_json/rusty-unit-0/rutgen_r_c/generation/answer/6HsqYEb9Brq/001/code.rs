// Answer 0

fn test_ignore_integer_invalid_leading_zero() -> Result<()> {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: TestRead { input: b"01".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.err.to_string(), "InvalidNumber");
    }

    Ok(())
}

#[test]
fn test_ignore_integer_invalid_leading_zero() {
    test_ignore_integer_invalid_leading_zero().unwrap();
}

fn test_ignore_integer_invalid_character() -> Result<()> {
    struct TestRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

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

        fn discard(&mut self) {
            if self.position < self.input.len() {
                self.position += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.position as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mut deserializer = Deserializer {
        read: TestRead { input: b"a".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.err.to_string(), "InvalidNumber");
    }

    Ok(())
}

#[test]
fn test_ignore_integer_invalid_character() {
    test_ignore_integer_invalid_character().unwrap();
}

