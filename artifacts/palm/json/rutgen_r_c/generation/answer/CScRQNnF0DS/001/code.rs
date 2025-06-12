// Answer 0

#[test]
fn test_position_valid() {
    struct MockIter {
        line: usize,
        col: usize,
    }

    impl MockIter {
        fn line(&self) -> usize {
            self.line
        }

        fn col(&self) -> usize {
            self.col
        }
    }

    struct MockIoRead {
        iter: MockIter,
    }

    impl Read<'static> for MockIoRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            Position {
                line: self.iter.line(),
                column: self.iter.col(),
            }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
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

    let mock_iter = MockIter { line: 5, col: 10 };
    let mut mock_io_read = MockIoRead { iter: mock_iter };

    let pos = mock_io_read.position();
    assert_eq!(pos.line, 5);
    assert_eq!(pos.column, 10);
}

#[test]
fn test_position_initial_state() {
    struct MockIter {
        line: usize,
        col: usize,
    }

    impl MockIter {
        fn line(&self) -> usize {
            self.line
        }

        fn col(&self) -> usize {
            self.col
        }
    }

    struct MockIoRead {
        iter: MockIter,
    }

    impl Read<'static> for MockIoRead {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
        
        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            Position {
                line: self.iter.line(),
                column: self.iter.col(),
            }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            0
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
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

    let mock_iter = MockIter { line: 0, col: 0 };
    let mut mock_io_read = MockIoRead { iter: mock_iter };

    let pos = mock_io_read.position();
    assert_eq!(pos.line, 0);
    assert_eq!(pos.column, 0);
}

