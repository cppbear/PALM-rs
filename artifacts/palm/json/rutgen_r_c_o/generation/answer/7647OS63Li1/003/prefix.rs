// Answer 0

#[test]
fn test_parse_integer_zero() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'0']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_positive() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'1', b'2']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_valid_large_positive() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'9', b'0', b'1']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_with_overflow() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_invalid_leading_zero() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'0', b'5']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_eof() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![]), 
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
}

#[test]
fn test_parse_integer_invalid_number() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'1', b'0', b'0']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(true);
}

#[test]
fn test_parse_integer_negative() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'-', b'1']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let _ = deserializer.parse_integer(false);
}

#[test]
fn test_parse_integer_invalid_large() {
    let mut deserializer = Deserializer { 
        read: MockRead::new(vec![b'1', b'2', b'9', b'9', b'9', b'9']),
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_integer(true);
    assert!(result.is_err());
}

// Mock implementation for testing
struct MockRead {
    data: Vec<u8>,
    index: usize,
}

impl MockRead {
    fn new(data: Vec<u8>) -> Self {
        MockRead { data, index: 0 }
    }
}

impl<'de> Read<'de> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.index < self.data.len() {
            let byte = self.data[self.index];
            self.index += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.index < self.data.len() {
            Ok(Some(self.data[self.index]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.index += 1;
    }

    fn position(&self) -> Position {
        Position { line: 0, column: self.index as u32 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: self.index as u32 }
    }

    fn byte_offset(&self) -> usize {
        self.index
    }

    fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        unimplemented!()
    }

    fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, [u8]>> {
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

