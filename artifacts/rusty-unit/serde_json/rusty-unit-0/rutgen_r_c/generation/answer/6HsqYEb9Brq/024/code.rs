// Answer 0

fn test_ignore_integer_valid_input() -> Result<()> {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let result = self.input[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            // Just a placeholder for position
            Position { line: 1, column: self.index as u32 }
        }

        fn peek_position(&self) -> Position {
            // Just a placeholder for position
            Position { line: 1, column: self.index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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
            V: Visitor<'_>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut mock_read = MockRead { input: b"42".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 8 };

    let result = deserializer.ignore_integer();
    assert_eq!(result, Ok(()));
    Ok(())
}

fn test_ignore_integer_with_leading_zero() -> Result<()> {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let result = self.input[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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
            V: Visitor<'_>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut mock_read = MockRead { input: b"0123".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 8 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
    Ok(())
}

fn test_ignore_integer_invalid_character() -> Result<()> {
    struct MockRead {
        input: Vec<u8>,
        index: usize,
    }

    impl Read<'_> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                let result = self.input[self.index];
                self.index += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.input.len() {
                Ok(Some(self.input[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            Position { line: 1, column: self.index as u32 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: self.index as u32 }
        }

        fn byte_offset(&self) -> usize {
            self.index
        }

        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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
            V: Visitor<'_>,
        {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {
            unimplemented!()
        }
    }

    let mut mock_read = MockRead { input: b"A1".to_vec(), index: 0 };
    let mut deserializer = Deserializer { read: mock_read, scratch: Vec::new(), remaining_depth: 8 };

    let result = deserializer.ignore_integer();
    assert!(result.is_err());
    Ok(())
}

