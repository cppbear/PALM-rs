// Answer 0

#[test]
fn test_next_with_valid_data() {
    struct MockReader {
        data: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data, position: 0 }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                let result = self.data[self.position];
                self.position += 1;
                Ok(Some(result))
            } else {
                Ok(None)
            }
        }

        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::zero() }
        fn peek_position(&self) -> Position { Position::zero() }
        fn byte_offset(&self) -> usize { self.position }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![1, 2, 3]);
    assert_eq!(reader.next().unwrap(), Some(1));
    assert_eq!(reader.next().unwrap(), Some(2));
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn test_next_with_empty_data() {
    struct MockReader {
        data: Vec<u8>,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            MockReader { data }
        }
    }

    impl Read<'static> for MockReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(self.data.get(0).copied())
        }

        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { Position::zero() }
        fn peek_position(&self) -> Position { Position::zero() }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = MockReader::new(vec![]);
    assert_eq!(reader.next().unwrap(), None);
}

