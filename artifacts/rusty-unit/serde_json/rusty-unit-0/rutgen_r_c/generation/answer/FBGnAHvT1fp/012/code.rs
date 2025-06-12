// Answer 0

#[test]
fn test_ignore_decimal_no_digits() {
    struct MockRead {
        bytes: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.bytes.len() {
                let byte = self.bytes[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.bytes.len() {
                Ok(Some(self.bytes[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            if self.index < self.bytes.len() {
                self.index += 1;
            }
        }

        fn position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: self.index }
        }

        fn byte_offset(&self) -> usize {
            self.index
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

        #[cfg(feature = "raw_value")]
        fn begin_raw_buffering(&mut self) {
            unimplemented!()
        }

        #[cfg(feature = "raw_value")]
        fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mock_read = MockRead { bytes: vec![b'.', b'e'], index: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 5 };

    let result = deserializer.ignore_decimal();
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.err, Error::syntax(ErrorCode::InvalidNumber, 0, 2));
    }
}

