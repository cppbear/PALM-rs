// Answer 0

#[test]
fn test_peek_position() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.position
        }

        fn peek_position(&self) -> Position {
            Position { line: self.position.line + 1, column: self.position.column + 1 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, failed: &mut bool) {}
    }

    let initial_position = Position { line: 5, column: 10 };
    let mut reader = TestReader { position: initial_position };

    let peeked_position = (&mut reader).peek_position();
    
    assert_eq!(peeked_position.line, initial_position.line + 1);
    assert_eq!(peeked_position.column, initial_position.column + 1);
}

