// Answer 0

#[test]
fn test_ignore_str_valid_case() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn new(slice: Vec<u8>) -> Self {
            Self { slice, index: 0 }
        }

        fn skip_to_escape(&mut self, _: bool) {
            // No-op for this test
        }

        fn ignore_escape(&mut self) -> Result<()> {
            // No-op for this test
            Ok(())
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return error(self, ErrorCode::EofWhileParsingString);
                }
                match self.slice[self.index] {
                    b'"' => {
                        self.index += 1;
                        return Ok(());
                    }
                    b'\\' => {
                        self.index += 1;
                        tri!(self.ignore_escape());
                    }
                    _ => {
                        return error(self, ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![b'a', b'b', b'"']);
    test_struct.index = 2; // Pointing to the double quote
    let result = test_struct.ignore_str();
    assert_eq!(result, Ok(()));
}

#[test]
fn test_ignore_str_with_escape() {
    struct TestStruct {
        slice: Vec<u8>,
        index: usize,
    }

    impl TestStruct {
        fn new(slice: Vec<u8>) -> Self {
            Self { slice, index: 0 }
        }

        fn skip_to_escape(&mut self, _: bool) {
            // No-op for this test
        }

        fn ignore_escape(&mut self) -> Result<()> {
            // No-op for this test
            Ok(())
        }

        fn ignore_str(&mut self) -> Result<()> {
            loop {
                self.skip_to_escape(true);
                if self.index == self.slice.len() {
                    return error(self, ErrorCode::EofWhileParsingString);
                }
                match self.slice[self.index] {
                    b'"' => {
                        self.index += 1;
                        return Ok(());
                    }
                    b'\\' => {
                        self.index += 1;
                        tri!(self.ignore_escape());
                    }
                    _ => {
                        return error(self, ErrorCode::ControlCharacterWhileParsingString);
                    }
                }
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![b'a', b'b', b'\\', b'"']);
    test_struct.index = 2; // Pointing to the escape character
    let result = test_struct.ignore_str();
    assert_eq!(result, Ok(()));
}

