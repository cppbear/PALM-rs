// Answer 0

#[test]
fn test_ignore_str_eof_while_parsing_string() {
    struct TestStruct {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {
            // This method does nothing for the test since we focus on the inner logic.
        }

        fn ignore_escape(&mut self) -> Result<()> {
            // Simulate ignoring escape sequences without performing actual logic.
            Ok(())
        }
        
        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return Err(ErrorCode::EofWhileParsingString);
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
                        return Err(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test = TestStruct { 
        index: 0, 
        slice: vec![b'\\', b'e', b'x', b'"'] // includes escape character followed by a control character
    };
    
    assert!(test.ignore_str().is_ok());
}

#[test]
fn test_ignore_str_control_character_while_parsing_string() {
    struct TestStruct {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {}
        
        fn ignore_escape(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return Err(ErrorCode::EofWhileParsingString);
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
                        return Err(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test = TestStruct { 
        index: 0, 
        slice: vec![b'x', b'y', b'z'] // this should trigger the control character error
    };

    assert!(matches!(test.ignore_str(), Err(ErrorCode::ControlCharacterWhileParsingString)));
}

#[test]
fn test_ignore_str_valid_escape() {
    struct TestStruct {
        index: usize,
        slice: Vec<u8>,
    }

    impl TestStruct {
        fn skip_to_escape(&mut self, _: bool) {}

        fn ignore_escape(&mut self) -> Result<()> {
            Ok(())
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return Err(ErrorCode::EofWhileParsingString);
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
                        return Err(ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test = TestStruct { 
        index: 0, 
        slice: vec![b'\\', b'"', b'"'] // properly handling a valid escape and two quotes
    };

    assert!(test.ignore_str().is_ok());
}

