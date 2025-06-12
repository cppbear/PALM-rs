// Answer 0

#[test]
fn test_byte_offset_valid_case() {
    struct TestReader {
        offset: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(0))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(0))
        }

        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            self.offset
        }
        
        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
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

    let mut reader = TestReader { offset: 100 };
    let offset = reader.byte_offset();
}

#[test]
fn test_byte_offset_zero_case() {
    struct TestReader {
        offset: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(0))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(0))
        }

        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            self.offset
        }
        
        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
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

    let mut reader = TestReader { offset: 0 };
    let offset = reader.byte_offset();
}

#[test]
fn test_byte_offset_max_case() {
    struct TestReader {
        offset: usize,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> {
            Ok(Some(0))
        }

        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(Some(0))
        }

        fn discard(&mut self) {}
        
        fn position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn peek_position(&self) -> Position {
            Position { line: 0, column: 0 }
        }

        fn byte_offset(&self) -> usize {
            self.offset
        }
        
        fn parse_str<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(
            &'s mut self,
            scratch: &'s mut Vec<u8>,
        ) -> Result<Reference<'static, 's, [u8]>> {
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

    let mut reader = TestReader { offset: usize::MAX };
    let offset = reader.byte_offset();
}

