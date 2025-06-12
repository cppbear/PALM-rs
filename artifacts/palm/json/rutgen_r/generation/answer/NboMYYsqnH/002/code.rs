// Answer 0

#[test]
fn test_do_deserialize_u128_valid() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = u128;

        fn visit_u128(self, value: u128) -> Result<Self::Value> {
            Ok(value)
        }
    }

    struct Deserializer {
        continue_flag: bool,
        has_error: bool,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.has_error {
                Err(ErrorCode::EofWhileParsingValue.into())
            } else {
                Ok(Some(b' '))
            }
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            if self.has_error {
                Err(ErrorCode::NumberOutOfRange.into())
            } else {
                buf.push_str("123");
                Ok(())
            }
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = Deserializer {
        continue_flag: true,
        has_error: false,
    };
    
    let visitor = Visitor;
    let result = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Ok(123));
}

#[test]
fn test_do_deserialize_u128_invalid_negative() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = u128;

        fn visit_u128(self, _value: u128) -> Result<Self::Value> {
            Err(ErrorCode::NumberOutOfRange.into())
        }
    }

    struct Deserializer {
        has_negative: bool,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(if self.has_negative { Some(b'-') } else { Some(b' ') })
        }

        fn scan_integer128(&mut self, buf: &mut String) -> Result<()> {
            buf.push_str("123");
            Ok(())
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = Deserializer {
        has_negative: true,
    };
    
    let visitor = Visitor;
    let result = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

#[test]
fn test_do_deserialize_u128_eof_error() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = u128;

        fn visit_u128(self, _value: u128) -> Result<Self::Value> {
            Ok(0)  // Not reached
        }
    }

    struct Deserializer {
        eof: bool,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            if self.eof {
                Err(ErrorCode::EofWhileParsingValue.into())
            } else {
                Ok(Some(b' '))
            }
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<()> {
            Ok(())
        }
        
        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = Deserializer {
        eof: true,
    };
    
    let visitor = Visitor;
    let result = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Err(ErrorCode::EofWhileParsingValue.into()));
}

#[test]
fn test_do_deserialize_u128_scan_error() {
    struct Visitor;

    impl<'any> de::Visitor<'any> for Visitor {
        type Value = u128;

        fn visit_u128(self, _value: u128) -> Result<Self::Value> {
            Ok(0)  // Not reached
        }
    }

    struct Deserializer {
        scan_error: bool,
    }

    impl Deserializer {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b' '))
        }

        fn scan_integer128(&mut self, _: &mut String) -> Result<()> {
            if self.scan_error {
                Err(ErrorCode::NumberOutOfRange.into())
            } else {
                Ok(())
            }
        }

        fn peek_error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn error(&self, code: ErrorCode) -> Error {
            Error::new(code)
        }

        fn fix_position(&self, err: Error) -> Error {
            err
        }
    }

    let mut deserializer = Deserializer {
        scan_error: true,
    };
    
    let visitor = Visitor;
    let result = deserializer.do_deserialize_u128(visitor);
    assert_eq!(result, Err(ErrorCode::NumberOutOfRange.into()));
}

