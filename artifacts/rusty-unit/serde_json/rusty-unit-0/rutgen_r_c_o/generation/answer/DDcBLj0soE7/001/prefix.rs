// Answer 0

#[test]
fn test_error_with_message_code() {
    let position = Position { line: 100, column: 200 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::Message(Box::from("Test Error Message")));
}

#[test]
fn test_error_with_io_code() {
    let position = Position { line: 50, column: 75 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::Io(std::io::Error::new(std::io::ErrorKind::Other, "IO Error")));
}

#[test]
fn test_error_with_eof_while_parsing_list() {
    let position = Position { line: 1, column: 1 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::EofWhileParsingList);
}

#[test]
fn test_error_with_eof_while_parsing_object() {
    let position = Position { line: 2, column: 10 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::EofWhileParsingObject);
}

#[test]
fn test_error_with_invalid_number() {
    let position = Position { line: 3, column: 15 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::InvalidNumber);
}

#[test]
fn test_error_with_expected_colon() {
    let position = Position { line: 4, column: 20 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::ExpectedColon);
}

#[test]
fn test_error_with_recursion_limit_exceeded() {
    let position = Position { line: 5, column: 25 };
    let read = MockRead { position };
    let deserializer = Deserializer { read, scratch: Vec::new(), remaining_depth: 10 };

    deserializer.error(ErrorCode::RecursionLimitExceeded);
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

    fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, '_, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, '_, [u8]>> {
        unimplemented!()
    }

    fn ignore_str(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        unimplemented!()
    }
}

