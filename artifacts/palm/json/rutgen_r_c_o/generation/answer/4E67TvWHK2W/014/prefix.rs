// Answer 0

#[test]
fn test_parse_decimal_overflow() {
    let mut deserializer = Deserializer {
        read: MyRead { data: b"12345678901234567890.0".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_decimal(false, 18446744073709551615, 5);
}

#[test]
fn test_parse_decimal_invalid_number() {
    let mut deserializer = Deserializer {
        read: MyRead { data: b"12345.0e".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_decimal(false, 10, 0);
}

#[test]
fn test_parse_decimal_no_digits_after_decimal() {
    let mut deserializer = Deserializer {
        read: MyRead { data: b"10.".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_decimal(true, 10, 2);
}

#[test]
fn test_parse_decimal_exponent_overflow() {
    let mut deserializer = Deserializer {
        read: MyRead { data: b"1e99999".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_decimal(false, 1, 0);
}

#[test]
fn test_parse_decimal_negative() {
    let mut deserializer = Deserializer {
        read: MyRead { data: b"-1234.5678".to_vec(), position: 0 },
        scratch: Vec::new(),
        remaining_depth: 0,
    };
    let result = deserializer.parse_decimal(false, 1234, 0);
}

struct MyRead {
    data: Vec<u8>,
    position: usize,
}

impl Read<'_> for MyRead {
    const should_early_return_if_failed: bool = false;

    fn next(&mut self) -> Result<Option<u8>> {
        if self.position < self.data.len() {
            let byte = self.data[self.position];
            self.position += 1;
            Ok(Some(byte))
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
        Position { line: 1, column: self.position as u32 }
    }

    fn peek_position(&self) -> Position {
        Position { line: 1, column: self.position as u32 }
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

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        unimplemented!()
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'_>,
    {
        unimplemented!()
    }

    fn set_failed(&mut self, _failed: &mut bool) {}
}

