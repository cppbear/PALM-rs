// Answer 0

#[test]
fn test_scan_integer128_zero_followed_by_non_digit() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'0', b'a']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.scan_integer128(&mut buf).unwrap();
}

#[test]
fn test_scan_integer128_zero_followed_by_space() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'0', b' ']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.scan_integer128(&mut buf).unwrap();
}

#[test]
fn test_scan_integer128_zero_followed_by_newline() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'0', b'\n']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.scan_integer128(&mut buf).unwrap();
}

#[test]
fn test_scan_integer128_one_followed_by_digits() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'1', b'2', b'3']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.scan_integer128(&mut buf).unwrap();
}

#[test]
fn test_scan_integer128_two_followed_by_digits() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'2', b'5', b'0']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    deserializer.scan_integer128(&mut buf).unwrap();
}

#[test]
fn test_scan_integer128_invalid_zero_followed_by_zero() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'0', b'0']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_invalid_char_after_zero() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'0', b'1']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_err());
}

#[test]
fn test_scan_integer128_invalid_non_digit_start() {
    let mut buf = String::new();
    let mut deserializer = Deserializer {
        read: FakeRead::new(vec![b'a']),
        scratch: Vec::new(),
        remaining_depth: 8,
    };
    let result = deserializer.scan_integer128(&mut buf);
    assert!(result.is_err());
}

struct FakeRead {
    input: Vec<u8>,
    position: usize,
}

impl FakeRead {
    fn new(input: Vec<u8>) -> Self {
        Self { input, position: 0 }
    }
}

impl Read<'_> for FakeRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position < self.input.len() {
            self.position += 1;
            Ok(Some(self.input[self.position - 1]))
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

    // Other methods can be left unimplemented for this mock
}

