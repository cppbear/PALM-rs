// Answer 0

fn test_do_deserialize_i128_positive() {
    struct Visitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = i128;

        fn visit_i128(self, value: i128) -> Result<Self::Value> {
            self.value = Some(value);
            Ok(value)
        }
    }

    struct Deserializer {
        input: &'static str,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulating whitespace parsing by returning Ok for the first character
            if self.input.starts_with(' ') {
                self.input = &self.input[1..];
                return Ok(Some(b' '));
            }
            Ok(None)
        }

        fn eat_char(&mut self) {
            self.input = &self.input[1..];
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            // Simulating scanning an integer
            buf.push_str("12345678901234567890");
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, _error: Error) -> Error {
            Error {}
        }
    }

    let mut deserializer = Deserializer {
        input: " - 12345678901234567890",
    };
    assert_eq!(
        deserializer.do_deserialize_i128(Visitor { value: None }).unwrap(),
        12345678901234567890
    );
}

#[should_panic]
fn test_do_deserialize_i128_negative() {
    struct Visitor {
        value: Option<i128>,
    }

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = i128;

        fn visit_i128(self, _value: i128) -> Result<Self::Value> {
            Err(Error {})
        }
    }

    struct Deserializer {
        input: &'static str,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            // Simulating whitespace parsing returning Ok for a character
            Ok(Some(b'-'))
        }

        fn eat_char(&mut self) {
            self.input = &self.input[1..];
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            // Simulating scanning an integer that triggers an error
            buf.push_str("invalid");
            Err(Error {})
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn error(&self, _code: ErrorCode) -> Error {
            Error {}
        }

        fn fix_position(&self, _error: Error) -> Error {
            Error {}
        }
    }

    let mut deserializer = Deserializer {
        input: " - invalid",
    };
    deserializer.do_deserialize_i128(Visitor { value: None }).unwrap();
}

