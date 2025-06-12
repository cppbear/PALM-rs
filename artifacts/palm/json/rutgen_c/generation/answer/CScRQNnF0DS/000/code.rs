// Answer 0

#[test]
fn test_position() {
    struct MockIter {
        line: usize,
        col: usize,
    }
    
    impl MockIter {
        fn new(line: usize, col: usize) -> Self {
            MockIter { line, col }
        }
        
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

    impl<'de> Read<'de> for MockIoRead {
        const should_early_return_if_failed: bool = true;
    
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
            Position {
                line: 0,
                column: 0,
            }
        }
        
        fn byte_offset(&self) -> usize {
            0
        }
    
        fn parse_str<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, str>> {
            unimplemented!()
        }
    
        fn parse_str_raw<'s>(
            &'s mut self,
            _scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'de, 's, [u8]>> {
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
    
    let mock_iter = MockIter::new(5, 10);
    let mock_reader = MockIoRead { iter: mock_iter };
    
    let position = mock_reader.position();
    
    assert_eq!(position.line, 5);
    assert_eq!(position.column, 10);
}

