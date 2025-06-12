// Answer 0

#[test]
fn test_peek_error_position_zero() {
    let position = Position { line: 0, column: 0 };
    let mut deserializer = Deserializer {
        read: MockRead { position },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let reason = ErrorCode::EofWhileParsingValue;
    deserializer.peek_error(reason);
}

#[test]
fn test_peek_error_position_boundary() {
    let position = Position { line: u32::MAX as usize, column: u32::MAX as usize };
    let mut deserializer = Deserializer {
        read: MockRead { position },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let reason = ErrorCode::ExpectedColon;
    deserializer.peek_error(reason);
}

#[test]
fn test_peek_error_position_midrange() {
    let position = Position { line: 123456789, column: 987654321 };
    let mut deserializer = Deserializer {
        read: MockRead { position },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let reason = ErrorCode::EofWhileParsingList;
    deserializer.peek_error(reason);
}

struct MockRead {
    position: Position,
}

impl Read<'_> for MockRead {
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
        self.position
    }

    fn byte_offset(&self) -> usize {
        0
    }

    fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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

