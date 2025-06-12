// Answer 0

#[test]
fn test_peek_with_valid_input() {
    struct TestReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(buffer: Vec<u8>) -> Self {
            Self { buffer, position: 0 }
        }
    }

    impl Read<'_> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                let byte = self.buffer[self.position];
                self.position += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.buffer.len() {
                Ok(Some(self.buffer[self.position]))
            } else {
                Ok(None)
            }
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { unimplemented!() }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = TestReader::new(vec![1, 2, 3]);
    assert_eq!(reader.peek().unwrap(), Some(1));
}

#[test]
fn test_peek_at_end_of_input() {
    struct EmptyReader;

    impl Read<'_> for EmptyReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut reader = EmptyReader;
    assert_eq!(reader.peek().unwrap(), None);
}

