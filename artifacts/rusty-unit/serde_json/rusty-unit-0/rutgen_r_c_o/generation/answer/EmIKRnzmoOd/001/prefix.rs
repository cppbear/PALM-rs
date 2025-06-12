// Answer 0

#[test]
fn test_scan_integer128_with_leading_zero() {
    let mut buf = String::new();
    let mut deserializer = Deserializer { read: &mut ReadMock { input: vec![b'0', b'0'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_with_non_digit_start() {
    let mut buf = String::new();
    let mut deserializer = Deserializer { read: &mut ReadMock { input: vec![b'-', b'1'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_with_invalid_number() {
    let mut buf = String::new();
    let mut deserializer = Deserializer { read: &mut ReadMock { input: vec![b'1', b'a'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.scan_integer128(&mut buf);
}

#[test]
fn test_scan_integer128_with_only_invalid_input() {
    let mut buf = String::new();
    let mut deserializer = Deserializer { read: &mut ReadMock { input: vec![b'x'], position: 0 }, scratch: Vec::new(), remaining_depth: 0 };
    let result = deserializer.scan_integer128(&mut buf);
}

struct ReadMock {
    input: Vec<u8>,
    position: usize,
}

impl Read<'_> for ReadMock {
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

    fn discard(&mut self) {}

    fn position(&self) -> Position {
        Position { line: 0, column: self.position }
    }

    fn peek_position(&self) -> Position {
        Position { line: 0, column: self.position }
    }

    fn byte_offset(&self) -> usize {
        self.position
    }

    fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, str>> unimplemented!();

    fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'_, 's, [u8]>> unimplemented!();

    fn ignore_str(&mut self) -> Result<()> {
        Ok(())
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        unimplemented!()
    }

    fn set_failed(&mut self, _failed: &mut bool) {}
}

