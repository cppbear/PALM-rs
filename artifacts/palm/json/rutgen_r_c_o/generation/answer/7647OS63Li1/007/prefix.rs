// Answer 0

#[test]
fn test_parse_integer_zero_leading() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_single_digit() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'1']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_multi_digit() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'2', b'5']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_overflow() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'5', b'0', b'1']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_invalid_leading_zero() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0', b'1']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let res = deserializer.parse_integer(true);
    assert!(res.is_err());
}

#[test]
fn test_parse_integer_invalid_character() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'a']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let res = deserializer.parse_integer(true);
    assert!(res.is_err());
}

#[test]
fn test_parse_integer_eof() {
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![]),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let res = deserializer.parse_integer(true);
    assert!(res.is_err());
}

struct MockRead {
    data: Vec<u8>,
    pos: usize,
}

impl MockRead {
    fn new(data: Vec<u8>) -> Self {
        MockRead { data, pos: 0 }
    }
}

impl Read<'_> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.pos < self.data.len() {
            let byte = self.data[self.pos];
            self.pos += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.pos < self.data.len() {
            Ok(Some(self.data[self.pos]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {}

    fn position(&self) -> Position {
        Position { line: 0, column: 0 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: 0 }
    }

    fn byte_offset(&self) -> usize {
        self.pos
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

    fn set_failed(&mut self, _failed: &mut bool) {
        unimplemented!()
    }
}

