// Answer 0

#[test]
fn test_end_with_ok_val_trailing_characters() {
    struct MockDeserializer {
        whitespace_parsed: bool,
        error_triggered: bool,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<()>, &'static str> {
            if self.whitespace_parsed {
                Ok(Some(()))
            } else {
                self.whitespace_parsed = true;
                Ok(None)
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> &'static str {
            "TrailingCharacters"
        }
    }

    let mut deserializer = MockDeserializer {
        whitespace_parsed: false,
        error_triggered: false,
    };
    
    let result = deserializer.end();
    assert_eq!(result, Err(deserializer.peek_error(ErrorCode::TrailingCharacters)));
}

#[test]
fn test_end_with_err() {
    struct MockDeserializer {
        should_error: bool,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<()>, &'static str> {
            if self.should_error {
                Err("Some error")
            } else {
                Ok(None)
            }
        }
    }

    let mut deserializer = MockDeserializer {
        should_error: true,
    };
    
    let result = deserializer.end();
    assert_eq!(result, Err("Some error"));
}

#[test]
fn test_end_with_ok_val_no_trailing_characters() {
    struct MockDeserializer {
        whitespace_parsed: bool,
    }

    impl MockDeserializer {
        fn parse_whitespace(&mut self) -> Result<Option<()>, &'static str> {
            if self.whitespace_parsed {
                Ok(Some(()))
            } else {
                self.whitespace_parsed = true;
                Ok(Some(()))
            }
        }

        fn peek_error(&self, _code: ErrorCode) -> &'static str {
            unreachable!()
        }
    }

    let mut deserializer = MockDeserializer {
        whitespace_parsed: false,
    };
    
    let result = deserializer.end();
    assert_eq!(result, Ok(()));
}

