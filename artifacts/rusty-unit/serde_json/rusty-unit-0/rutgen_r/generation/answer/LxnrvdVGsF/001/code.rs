// Answer 0

#[test]
fn test_ignore_str_eof_while_parsing_string() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {
            // No-op for the purpose of this test
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::EofWhileParsingString)
        }

        fn ignore_escape(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return self.error(ErrorCode::EofWhileParsingString);
                }
                match self.slice[self.index] {
                    b'"' => {
                        self.index += 1;
                        return Ok(());
                    }
                    b'\\' => {
                        self.index += 1;
                        self.ignore_escape()?;
                    }
                    _ => {
                        return self.error(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test_struct = TestStruct {
        slice: vec![],
        index: 0,
    };

    assert_eq!(test_struct.ignore_str(), Err(ErrorCode::EofWhileParsingString));
}

#[test]
fn test_ignore_str_control_character_while_parsing_string() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {
            // No-op for the purpose of this test
        }

        fn error(&self, _: ErrorCode) -> Result<()> {
            Err(ErrorCode::ControlCharacterWhileParsingString)
        }

        fn ignore_escape(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return self.error(ErrorCode::EofWhileParsingString);
                }
                match self.slice[self.index] {
                    b'"' => {
                        self.index += 1;
                        return Ok(());
                    }
                    b'\\' => {
                        self.index += 1;
                        self.ignore_escape()?;
                    }
                    _ => {
                        return self.error(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test_struct = TestStruct {
        slice: vec![b'\x01'],
        index: 0,
    };

    assert_eq!(test_struct.ignore_str(), Err(ErrorCode::ControlCharacterWhileParsingString));
}

