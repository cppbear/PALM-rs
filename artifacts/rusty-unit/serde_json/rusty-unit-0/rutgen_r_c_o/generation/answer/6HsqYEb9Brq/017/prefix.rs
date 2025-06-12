// Answer 0

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0', b'0']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_single_digit() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'1', b'0']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_multiple_digits() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'2', b'3', b'4']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_multiple_leading_zero() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0', b'1']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_non_numeric() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'a']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

struct MockRead {
    data: Vec<u8>,
    position: usize,
}

impl MockRead {
    fn new(data: Vec<u8>) -> Self {
        Self { data, position: 0 }
    }
}

impl Read<'_> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position < self.data.len() {
            self.position += 1;
            Ok(Some(self.data[self.position - 1]))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.position < self.data.len() {
            Ok(Some(self.data[self.position]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.position += 1;
    }

    fn position(&self) -> Position {
        Position { line: 1, column: self.position + 1 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 1, column: self.position + 1 }
    }

    fn byte_offset(&self) -> usize {
        self.position
    }

    fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> {
        unimplemented!()
    }

    fn ignore_str(&mut self) -> Result<()> {
        unimplemented!()
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        unimplemented!()
    }
}

