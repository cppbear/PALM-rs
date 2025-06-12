// Answer 0

#[test]
fn test_ignore_str_valid() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {
            // No-op for this test
        }

        fn ignore_escape(&mut self) -> Result<()> {
            // Mock implementation for testing
            self.index += 1; 
            Ok(())
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingString)
        }
    }

    let mut test_struct = TestStruct {
        slice: b"{\"text\":\"example\\\"text\"}".to_vec(),
        index: 1, // Start after the initial quote
    };

    assert!(test_struct.ignore_str().is_ok());
}

#[test]
fn test_ignore_str_eof() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {
            // No-op for this test
        }

        fn ignore_escape(&mut self) -> Result<()> {
            // No-op for this test
            Ok(())
        }

        fn error(&self, code: ErrorCode) -> Result<()> {
            match code {
                ErrorCode::EofWhileParsingString => Err(ErrorCode::EofWhileParsingString),
                _ => unreachable!(),
            }
        }
    }

    let mut test_struct = TestStruct {
        slice: b"\"test".to_vec(), // Missing closing quote
        index: 0,
    };

    assert!(matches!(test_struct.ignore_str(), Err(ErrorCode::EofWhileParsingString)));
}

#[test]
fn test_ignore_str_control_character() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {
            // No-op for this test
        }

        fn ignore_escape(&mut self) -> Result<()> {
            // No-op for this test
            Ok(())
        }

        fn error(&self, code: ErrorCode) -> Result<()> {
            match code {
                ErrorCode::ControlCharacterWhileParsingString => Err(ErrorCode::ControlCharacterWhileParsingString),
                _ => unreachable!(),
            }
        }
    }

    let mut test_struct = TestStruct {
        slice: b"\"test\x00text\"".to_vec(), // Contains null character
        index: 0,
    };

    assert!(matches!(test_struct.ignore_str(), Err(ErrorCode::ControlCharacterWhileParsingString)));
}

