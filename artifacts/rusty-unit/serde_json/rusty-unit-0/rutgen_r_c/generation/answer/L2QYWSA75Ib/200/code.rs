// Answer 0

fn test_ignore_value_correct_handling() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
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
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u32, 1)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u32, 1)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::from_mut_str(""))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::from_mut_bytes(&[]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
            where V: Visitor<'de> {
            Ok(())
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead {
        input: b"true, false, null, {\"key\": \"value\"}".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 128,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    // Expected to succeed and clear the scratch
    assert!(deserializer.ignore_value().is_ok());

    // Now we simulate an invalid input subsequently
    deserializer.read.input = b"invalid_value".to_vec();
    deserializer.read.position = 0;

    // Expected to return an error due to invalid input
    assert!(matches!(deserializer.ignore_value(), Err(_)));
}

fn test_ignore_value_unexpected_token() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl<'de> Read<'de> for MockRead {
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
            self.position += 1;
        }

        fn position(&self) -> Position {
            Position::new(self.position as u32, 1)
        }

        fn peek_position(&self) -> Position {
            Position::new(self.position as u32, 1)
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            Ok(Reference::from_mut_str(""))
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            Ok(Reference::from_mut_bytes(&[]))
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            Ok(0)
        }

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {}

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
            where V: Visitor<'de> {
            Ok(())
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut mock_read = MockRead {
        input: b"unexpected_token".to_vec(),
        position: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_read,
        scratch: Vec::new(),
        remaining_depth: 128,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    // Expected to return an error due to an unexpected token
    assert!(matches!(deserializer.ignore_value(), Err(_)));
}

