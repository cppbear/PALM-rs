// Answer 0

#[test]
fn test_peek_position_initial() {
    struct TestRead {
        position: Position,
    }
    
    impl private::Sealed for TestRead {}
    
    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = true;
        
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
            self.position()
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

    let read = TestRead { position: Position { line: 1, column: 1 } };
    let pos = read.peek_position();
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 1);
}

#[test]
fn test_peek_position_after_change() {
    struct TestRead {
        position: Position,
    }
    
    impl private::Sealed for TestRead {}
    
    impl Read<'static> for TestRead {
        const should_early_return_if_failed: bool = true;
        
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
            self.position()
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

    let mut read = TestRead { position: Position { line: 1, column: 1 } };
    read.position = Position { line: 2, column: 5 };
    let pos = read.peek_position();
    assert_eq!(pos.line, 2);
    assert_eq!(pos.column, 5);
}

