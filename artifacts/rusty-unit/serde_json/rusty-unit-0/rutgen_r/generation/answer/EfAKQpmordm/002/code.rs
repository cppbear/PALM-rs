// Answer 0

#[test]
fn test_ignore_str_return_ok_on_double_quote() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }

    fn is_escape(ch: u8, _: bool) -> bool {
        ch == b'\\'
    }

    fn ignore_escape(_: &mut MockReader) -> Result<()> {
        Ok(())
    }

    fn error(_: &MockReader, _: ErrorCode) -> Result<()> {
        panic!("Unexpected control character");
    }

    let mut reader = MockReader::new(vec![b'"']);
    let result = reader.ignore_str();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_str_handle_escape() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }

    fn is_escape(ch: u8, _: bool) -> bool {
        ch == b'\\'
    }

    fn ignore_escape(_: &mut MockReader) -> Result<()> {
        Ok(())
    }
    
    fn error(_: &MockReader, _: ErrorCode) -> Result<()> {
        panic!("Unexpected control character");
    }

    let mut reader = MockReader::new(vec![b'\\', b'"']);
    let result = reader.ignore_str();
    assert!(result.is_ok());
}

#[test]
fn test_ignore_str_control_character() {
    struct MockReader {
        data: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(data: Vec<u8>) -> Self {
            Self { data, index: 0 }
        }

        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let ch = self.data[self.index];
                self.index += 1;
                Ok(ch)
            } else {
                Err(ErrorCode::EndOfFile)
            }
        }
    }

    fn is_escape(ch: u8, _: bool) -> bool {
        ch == b'\\'
    }

    fn ignore_escape(_: &mut MockReader) -> Result<()> {
        Ok(())
    }

    fn error(_: &MockReader, _: ErrorCode) -> Result<()> {
        Err(ErrorCode::ControlCharacterWhileParsingString)
    }

    let mut reader = MockReader::new(vec![b'\x01']);
    let result = reader.ignore_str();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), ErrorCode::ControlCharacterWhileParsingString);
}

