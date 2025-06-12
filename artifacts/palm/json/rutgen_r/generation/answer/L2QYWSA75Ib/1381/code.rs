// Answer 0

fn test_ignore_value_valid_true() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            // Simulate whitespace parsing, returning a valid eof end
            Ok(Some(b'n'))
        }
        
        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), ErrorCode> {
            Ok(())
        }
        
        fn ignore_integer(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn read_ignore_str(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::ExpectedSomeValue
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    assert_eq!(test_struct.ignore_value(), Ok(()));
}

fn test_ignore_value_invalid_ident() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(Some(b'n')) // returns valid character
        }
        
        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), ErrorCode> {
            Err(ErrorCode::ExpectedSomeValue) // simulating an error
        }
        
        fn ignore_integer(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn read_ignore_str(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> ErrorCode {
            ErrorCode::ExpectedSomeValue
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    assert_eq!(test_struct.ignore_value(), Err(ErrorCode::ExpectedSomeValue));
}

fn test_ignore_value_eof() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(None) // simulate EOF
        }
        
        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), ErrorCode> {
            Ok(())
        }
        
        fn ignore_integer(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn read_ignore_str(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn peek_error(&self, code: ErrorCode) -> ErrorCode {
            code
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    assert_eq!(test_struct.ignore_value(), Err(ErrorCode::EofWhileParsingValue));
}

