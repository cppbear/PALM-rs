// Answer 0

#[test]
fn test_position_method() {
    struct TestRead;

    impl<'de> Read<'de> for TestRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 1, column: 1 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 1, column: 1 }
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut test_read = TestRead;
    let pos = test_read.position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

#[test]
fn test_position_with_different_values() {
    struct AnotherTestRead;

    impl<'de> Read<'de> for AnotherTestRead {
        const should_early_return_if_failed: bool = true;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }

        fn discard(&mut self) {}

        fn position(&self) -> Position {
            Position { line: 2, column: 5 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 2, column: 5 }
        }

        fn byte_offset(&self) -> usize {
            10
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut another_test_read = AnotherTestRead;
    let pos = another_test_read.position();
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 5);
}

