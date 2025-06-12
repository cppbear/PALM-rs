// Answer 0

fn test_parse_number_positive_zero() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }

        fn peek_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_decimal(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            self.eat_char(); // simulate eating a character
            Ok(1.0) // simulate a valid decimal return
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _starting_exp: i32) -> Result<f64> {
            Err(Error::custom("Parse error")) // simulate an error
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) { unimplemented!() }
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { unimplemented!() }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) { unimplemented!() }
    }

    let mut reader = TestReader::new(vec![b'1', b'.', b'0']);
    let result = reader.parse_number(true, 10);
    assert!(result.is_ok());
}

fn test_parse_number_negative_underflow() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }

        fn peek_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_decimal(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            self.eat_char(); // simulate eating a character
            Ok(1.0) // simulate a valid decimal return
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _starting_exp: i32) -> Result<f64> {
            Ok(1.0) // simulate successful parsing of exponent
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) { unimplemented!() }
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { unimplemented!() }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) { unimplemented!() }
    }

    let mut reader = TestReader::new(vec![b'-', b'0']);
    let result = reader.parse_number(false, 0);
    assert!(result.is_err());
}

fn test_parse_number_large_positive() {
    struct TestReader {
        data: Vec<u8>,
        position: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, position: 0 }
        }

        fn peek_or_null(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }

        fn eat_char(&mut self) {
            if self.position < self.data.len() {
                self.position += 1;
            }
        }

        fn parse_decimal(&mut self, _positive: bool, _significand: u64, _exponent: i32) -> Result<f64> {
            Ok(1e10) // return a valid decimal
        }

        fn parse_exponent(&mut self, _positive: bool, _significand: u64, _starting_exp: i32) -> Result<f64> {
            Ok(1e5) // return a valid exponent
        }
    }

    impl Read<'static> for TestReader {
        const should_early_return_if_failed: bool = false;

        fn next(&mut self) -> Result<Option<u8>> { unimplemented!() }
        fn discard(&mut self) { unimplemented!() }
        fn position(&self) -> Position { unimplemented!() }
        fn peek_position(&self) -> Position { unimplemented!() }
        fn byte_offset(&self) -> usize { unimplemented!() }
        fn parse_str<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> { unimplemented!() }
        fn parse_str_raw<'s>(&'s mut self, _scratch: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> { unimplemented!() }
        fn ignore_str(&mut self) -> Result<()> { unimplemented!() }
        fn decode_hex_escape(&mut self) -> Result<u16> { unimplemented!() }
        fn set_failed(&mut self, _failed: &mut bool) { unimplemented!() }
    }

    let mut reader = TestReader::new(vec![b'9', b'e']);
    let result = reader.parse_number(true, 100);
    assert!(result.is_ok());
}

