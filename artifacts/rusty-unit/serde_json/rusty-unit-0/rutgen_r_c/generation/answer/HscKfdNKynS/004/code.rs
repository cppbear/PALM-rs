// Answer 0

fn test_parse_str_bytes_valid() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 } // simplified for this context
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 } // simplified for this context
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn begin_raw_buffering(&mut self) {}

        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'_>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![b'"', b'a', b'b', b'\\', b'c', b'"']);
    let mut scratch = Vec::new();

    let result = reader.parse_str_bytes::<(), _>(&mut scratch, true, |_, _| Ok(()));
    assert!(result.is_ok());
}

fn test_parse_str_bytes_invalid_control_character() {
    struct TestReader {
        bytes: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(bytes: Vec<u8>) -> Self {
            Self { bytes, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                let byte = self.bytes[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.bytes.len() {
                Ok(Some(self.bytes[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 0, column: 0 } // simplified for this context
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 } // simplified for this context
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'_, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn begin_raw_buffering(&mut self) {}

        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'_>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![b'"', b'a', b'\x01', b'c', b'"']); // Control character
    let mut scratch = Vec::new();

    let result = reader.parse_str_bytes::<(), _>(&mut scratch, true, |_, _| Ok(()));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().code, ErrorCode::ControlCharacterWhileParsingString);
}

