// Answer 0

#[test]
fn test_position() {
    struct MockRead {
        position: Position,
    }

    impl<'de> Read<'de> for MockRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position.clone()
        }

        fn peek_position(&self) -> Position {
            self.position.clone()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!("MockRead does not support parse_str")
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!("MockRead does not support parse_str_raw")
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!("MockRead does not support ignore_str")
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!("MockRead does not support decode_hex_escape")
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mock_read = MockRead {
        position: Position { line: 5, column: 10 },
    };
    assert_eq!(mock_read.position().line, 5);
    assert_eq!(mock_read.position().column, 10);
}

