// Answer 0

#[test]
fn test_parse_ident_correct_ident() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'a', b'b', b'c']),
        scratch: vec![],
        remaining_depth: 0,
    };
    let ident = b"abc";
    let result = deserializer.parse_ident(ident);
}

#[test]
fn test_parse_ident_incorrect_ident() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'a', b'b', b'd']),
        scratch: vec![],
        remaining_depth: 0,
    };
    let ident = b"abc";
    let result = deserializer.parse_ident(ident);
}

#[test]
fn test_parse_ident_eof_error() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![]),
        scratch: vec![],
        remaining_depth: 0,
    };
    let ident = b"abc";
    let result = deserializer.parse_ident(ident);
}

#[test]
fn test_parse_ident_multiple_chars() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'h', b'e', b'l', b'l', b'o']),
        scratch: vec![],
        remaining_depth: 0,
    };
    let ident = b"hello";
    let result = deserializer.parse_ident(ident);
}

#[test]
fn test_parse_ident_long_ident_with_error() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b't', b'e', b's', b't', b'o']),
        scratch: vec![],
        remaining_depth: 0,
    };
    let ident = b"test";
    let result = deserializer.parse_ident(ident);
}

struct MockRead {
    bytes: Vec<u8>,
    position: usize,
}

impl MockRead {
    fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, position: 0 }
    }
}

impl Read<'_> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position >= self.bytes.len() {
            Ok(None)
        } else {
            let byte = self.bytes[self.position];
            self.position += 1;
            Ok(Some(byte))
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.position >= self.bytes.len() {
            Ok(None)
        } else {
            Ok(Some(self.bytes[self.position]))
        }
    }

    fn discard(&mut self) {}

    fn position(&self) -> Position {
        Position { line: 0, column: self.position as u64 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: self.position as u64 }
    }

    fn byte_offset(&self) -> usize {
        self.position
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
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

