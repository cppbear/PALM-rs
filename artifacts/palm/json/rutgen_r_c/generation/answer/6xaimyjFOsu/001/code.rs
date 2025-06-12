// Answer 0

#[test]
fn test_peek_position() {
    struct TestDelegate {
        position: Position,
    }

    impl TestDelegate {
        pub fn new(line: usize, column: usize) -> Self {
            TestDelegate {
                position: Position { line, column },
            }
        }
    }

    impl Deref for TestDelegate {
        type Target = Position;

        fn deref(&self) -> &Self::Target {
            &self.position
        }
    }

    struct TestStrRead<'a> {
        delegate: TestDelegate,
    }

    impl<'a> Read<'a> for TestStrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.delegate.position
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let position = Position { line: 10, column: 5 };
    let delegate = TestDelegate::new(position.line, position.column);
    let mut str_read = TestStrRead { delegate };

    let result = str_read.peek_position();
    assert_eq!(result.line, position.line);
    assert_eq!(result.column, position.column);
}

#[test]
fn test_peek_position_with_edge_case() {
    struct TestDelegate {
        position: Position,
    }

    impl TestDelegate {
        pub fn new(line: usize, column: usize) -> Self {
            TestDelegate {
                position: Position { line, column },
            }
        }
    }

    impl Deref for TestDelegate {
        type Target = Position;

        fn deref(&self) -> &Self::Target {
            &self.position
        }
    }

    struct TestStrRead<'a> {
        delegate: TestDelegate,
    }

    impl<'a> Read<'a> for TestStrRead<'a> {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn discard(&mut self) {}

        fn position(&self) -> Position {
            self.delegate.position
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, [u8]>> {
            unimplemented!()
        }

        fn ignore_str(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn decode_hex_escape(&mut self) -> Result<u16> {
            unimplemented!()
        }

        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let position = Position { line: 0, column: 0 };
    let delegate = TestDelegate::new(position.line, position.column);
    let mut str_read = TestStrRead { delegate };

    let result = str_read.peek_position();
    assert_eq!(result.line, position.line);
    assert_eq!(result.column, position.column);
}

