// Answer 0

#[test]
fn test_ignore_integer_leading_zero_invalid() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"01"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_leading_zero_valid() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"0"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_single_digit() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"1"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_valid_multiple_digits() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"12345"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_char() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"a"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_with_decimal() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"123.45"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_with_exponent() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"123e10"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

#[test]
fn test_ignore_integer_invalid_after_zero() {
    let mut deserializer = Deserializer {
        read: MockRead::new(b"00"),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.ignore_integer();
}

// Mock implementations for testing
struct MockRead {
    data: Vec<u8>,
    position: usize,
}

impl MockRead {
    fn new(data: &[u8]) -> Self {
        Self {
            data: data.to_vec(),
            position: 0,
        }
    }
}

impl Read<'_> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position < self.data.len() {
            let val = self.data[self.position];
            self.position += 1;
            Ok(Some(val))
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
        Position { line: 0, column: self.position }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: self.position }
    }

    fn byte_offset(&self) -> usize {
        self.position
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

    fn set_failed(&mut self, _: &mut bool) {}
}

