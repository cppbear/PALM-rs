// Answer 0

fn test_ignore_exponent_valid_positive() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    impl MockRead {
        fn new(input: Vec<u8>) -> Self {
            Self { input, position: 0 }
        }
    }

    impl Read<'static> for MockRead {
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
            Position { line: 0, column: self.position }
        }

        fn peek_position(&self) -> Position {
            self.position()
        }

        fn byte_offset(&self) -> usize {
            self.position
        }

        fn parse_str<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, str>> {
            unimplemented!()
        }

        fn parse_str_raw<'s>(&'s mut self, _: &'s mut Vec<u8>) -> Result<Reference<'static, 's, [u8]>> {
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
        fn end_raw_buffering<V>(&mut self, _: V) -> Result<V::Value>
            where V: Visitor<'static> {
            unimplemented!()
        }

        fn set_failed(&mut self, _: &mut bool) {}
    }

    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'e', b'+', b'2']),
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_ok());
}

fn test_ignore_exponent_valid_negative() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    // Reusing MockRead from the previous test

    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'e', b'-', b'5']),
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_ok());
}

fn test_ignore_exponent_invalid_no_digit() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    // Reusing MockRead from the previous test

    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'e', b'+']),
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_err());
}

fn test_ignore_exponent_invalid_non_digit() {
    struct MockRead {
        input: Vec<u8>,
        position: usize,
    }

    // Reusing MockRead from the previous test

    let mut deserializer = Deserializer {
        read: MockRead::new(vec![b'e', b'+', b'A']),
        scratch: Vec::new(),
        remaining_depth: 10,
        #[cfg(feature = "float_roundtrip")]
        single_precision: false,
        #[cfg(feature = "unbounded_depth")]
        disable_recursion_limit: false,
    };

    let result = deserializer.ignore_exponent();
    assert!(result.is_err());
}

