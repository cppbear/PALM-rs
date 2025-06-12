// Answer 0

#[test]
fn test_ignore_value_with_true() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b't'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Err {
            Err {}
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    let result = test_struct.ignore_value();
    assert_eq!(result, Err(test_struct.peek_error(ErrorCode::ExpectedColon)));
}

#[test]
fn test_ignore_value_with_object_end() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'}'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Err {
            Err {}
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    let result = test_struct.ignore_value();
    assert_eq!(result, Err(test_struct.peek_error(ErrorCode::ExpectedColon)));
}

#[test]
#[should_panic]
fn test_ignore_value_with_invalid_char() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b'x')) // Invalid character
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Err {
            Err {}
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    test_struct.ignore_value();
}

#[test]
fn test_ignore_value_with_list_end() {
    struct TestStruct {
        scratch: Vec<u8>,
    }

    impl TestStruct {
        fn parse_whitespace(&mut self) -> Result<Option<u8>> {
            Ok(Some(b']'))
        }

        fn eat_char(&mut self) {}

        fn parse_ident(&mut self, _ident: &[u8]) -> Result<()> {
            Ok(())
        }

        fn read(&self) -> &Self {
            self
        }

        fn ignore_str(&self) -> Result<()> {
            Ok(())
        }

        fn peek_error(&self, _code: ErrorCode) -> Err {
            Err {}
        }

        fn ignore_integer(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut test_struct = TestStruct { scratch: Vec::new() };
    let result = test_struct.ignore_value();
    assert_eq!(result, Ok(()));
}

