// Answer 0

fn test_end_map_returns_ok_on_closing_brace() {
    struct MockReader {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                let val = self.values[self.index];
                self.index += 1;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                Ok(Some(self.values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockReader {
        values: vec![b' ', b'}'],
        index: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert_eq!(result, Ok(()));
}

fn test_end_map_returns_err_on_trailing_comma() {
    struct MockReader {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                let val = self.values[self.index];
                self.index += 1;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                Ok(Some(self.values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockReader {
        values: vec![b' ', b','],
        index: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert!(result.is_err());
}

fn test_end_map_returns_err_on_trailing_characters() {
    struct MockReader {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                let val = self.values[self.index];
                self.index += 1;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                Ok(Some(self.values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockReader {
        values: vec![b' ', b'a'],
        index: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert!(result.is_err());
}

fn test_end_map_returns_err_on_eof() {
    struct MockReader {
        values: Vec<u8>,
        index: usize,
    }

    impl<'de> Read<'de> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                let val = self.values[self.index];
                self.index += 1;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.index < self.values.len() {
                Ok(Some(self.values[self.index]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {
            self.index += 1;
        }

        fn position(&self) -> Position {
            unimplemented!()
        }

        fn peek_position(&self) -> Position {
            unimplemented!()
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
            Ok(())
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }
    }

    let mock_reader = MockReader {
        values: vec![],
        index: 0,
    };

    let mut deserializer = Deserializer {
        read: mock_reader,
        scratch: vec![],
        remaining_depth: 0,
    };

    let result = deserializer.end_map();
    assert!(result.is_err());
}

