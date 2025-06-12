// Answer 0

#[test]
fn test_scan_integer128_single_zero() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0']),
        scratch: vec![],
        remaining_depth: 8,
    };
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_valid_integer() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'1', b'2', b'3']),
        scratch: vec![],
        remaining_depth: 8,
    };
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_invalid_leading_zero() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0', b'1']),
        scratch: vec![],
        remaining_depth: 8,
    };
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_empty_input() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![]),
        scratch: vec![],
        remaining_depth: 8,
    };
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_only_zero() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'0']),
        scratch: vec![],
        remaining_depth: 8,
    };
    let _ = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_multiple_digits() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'4', b'5', b'6']),
        scratch: vec![],
        remaining_depth: 8,
    };
    let _ = deserializer.scan_integer128(&mut buf);
}

struct MockRead {
    input: Vec<u8>,
    position: usize,
}

impl MockRead {
    fn new(input: Vec<u8>) -> Self {
        MockRead { input, position: 0 }
    }
}

impl Read<'_> for MockRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position < self.input.len() {
            let byte = self.input[self.position];
            self.position += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        if self.position < self.input.len() {
            Ok(Some(self.input[self.position]))
        } else {
            Ok(None)
        }
    }

    fn discard(&mut self) {
        self.position += 1;
    }

    fn position(&self) -> Position {
        Position::new(self.position)
    }

    fn peek_position(&self) -> Position {
        Position::new(self.position)
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
}

