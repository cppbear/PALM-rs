// Answer 0

fn test_ignore_value_return_err_if_whitespace_error() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Err(ErrorCode::EofWhileParsingValue)
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn peek_error(&self, error: ErrorCode) -> ErrorCode {
            error
        }
    }

    let mut test_struct = TestStruct { scratch: vec![] };
    let result = test_struct.ignore_value();
    
    assert!(result.is_err());
}

fn test_ignore_value_return_err_if_ident_error() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(Some(b'n'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), ErrorCode> {
            Err(ErrorCode::ExpectedSomeValue)
        }

        fn ignore_integer(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn peek_error(&self, error: ErrorCode) -> ErrorCode {
            error
        }
    }

    let mut test_struct = TestStruct { scratch: vec![] };
    let result = test_struct.ignore_value();
    
    assert!(result.is_err());
}

fn test_ignore_value_return_ok_on_successful_parsing() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>, ErrorCode> {
            Ok(Some(b'n'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn ignore_integer(&mut self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<(), ErrorCode> {
            Ok(())
        }

        fn peek_error(&self, _error: ErrorCode) -> ErrorCode {
            ErrorCode::ExpectedSomeValue
        }
    }

    let mut test_struct = TestStruct { scratch: vec![] };
    let result = test_struct.ignore_value();
    
    assert!(result.is_ok());
}

