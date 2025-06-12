// Answer 0

#[test]
fn test_peek_position_minimum() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { position: Position { line: 0, column: 0 } };
    let pos = reader.peek_position();
}

#[test]
fn test_peek_position_mid_range() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { position: Position { line: 500, column: 500 } };
    let pos = reader.peek_position();
}

#[test]
fn test_peek_position_maximum() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { position: Position { line: 1000, column: 1000 } };
    let pos = reader.peek_position();
}

#[test]
#[should_panic]
fn test_peek_position_invalid_line() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { assert!(self.position.line <= 1000); self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { position: Position { line: 1001, column: 0 } };
    let pos = reader.peek_position();
}

#[test]
#[should_panic]
fn test_peek_position_invalid_column() {
    struct TestReader {
        position: Position,
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn peek(&mut self) -> Result<Option<u8>> { Ok(None) }
        fn discard(&mut self) {}
        fn position(&self) -> Position { self.position }
        fn peek_position(&self) -> Position { assert!(self.position.column <= 1000); self.position }
        fn byte_offset(&self) -> usize { 0 }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { Ok(()) }
        fn decode_hex_escape(&mut self) -> Result<u16> { Ok(0) }
        fn set_failed(&mut self, _failed: &mut bool) {}
    }

    let mut reader = TestReader { position: Position { line: 0, column: 1001 } };
    let pos = reader.peek_position();
}

