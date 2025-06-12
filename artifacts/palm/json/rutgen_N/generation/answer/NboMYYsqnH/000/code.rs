// Answer 0

#[test]
fn test_do_deserialize_u128_positive() {
    struct Visitor {
        value: Option<u128>,
    }

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        data: &'static str,
        index: usize,
    }

    impl MockDeserializer {
        fn new(data: &'static str) -> Self {
            Self { data, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Option<u8> {
            if self.index < self.data.len() {
                let byte = self.data.as_bytes()[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            while self.index < self.data.len() {
                let byte = self.data.as_bytes()[self.index];
                if !byte.is_ascii_digit() {
                    break;
                }
                buf.push(byte as char);
                self.index += 1;
            }
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("error")
        }
        
        fn error(&self, _code: ErrorCode) -> Error {
            Error::new("error")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new("12345");
    let visitor = Visitor { value: None };
    let result = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Ok(12345));
}

#[test]
fn test_do_deserialize_u128_negative() {
    struct Visitor {
        value: Option<u128>,
    }

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = u128;

        fn visit_u128(self, _value: u128) -> Result<Self::Value> {
            Err(Error::new("error"))
        }
    }

    struct MockDeserializer {
        data: &'static str,
        index: usize,
    }

    impl MockDeserializer {
        fn new(data: &'static str) -> Self {
            Self { data, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Option<u8> {
            if self.index < self.data.len() {
                let byte = self.data.as_bytes()[self.index];
                self.index += 1;
                Some(byte)
            } else {
                None
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            while self.index < self.data.len() {
                let byte = self.data.as_bytes()[self.index];
                if !byte.is_ascii_digit() {
                    break;
                }
                buf.push(byte as char);
                self.index += 1;
            }
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("error")
        }
        
        fn error(&self, _code: ErrorCode) -> Error {
            Error::new("error")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new("12345");
    let visitor = Visitor { value: None };
    let result = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Err(Error::new("error")));
}

#[test]
#[should_panic]
fn test_do_deserialize_u128_eof() {
    struct Visitor {
        value: Option<u128>,
    }

    impl<'a> de::Visitor<'a> for Visitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct MockDeserializer {
        data: &'static str,
        index: usize,
    }

    impl MockDeserializer {
        fn new(data: &'static str) -> Self {
            Self { data, index: 0 }
        }

        fn parse_whitespace(&mut self) -> Option<u8> {
            None
        }

        fn scan_integer128(&mut self, _buf: &mut String) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Error {
            Error::new("error")
        }
        
        fn error(&self, _code: ErrorCode) -> Error {
            Error::new("error")
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = MockDeserializer::new("");
    let visitor = Visitor { value: None };
    let _ = deserializer.do_deserialize_u128(visitor);
}

