// Answer 0

#[test]
fn test_parse_str_bytes_success() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(ErrorCode::UnexpectedEof) // Simulate EOF
            }
        }

        fn parse_escape(&mut self, _validate: bool, scratch: &mut Vec<u8>) -> Result<()> {
            // Simulate escape sequence parsing
            if self.index + 1 < self.data.len() {
                scratch.push(self.data[self.index]);
                self.index += 1;
                Ok(())
            } else {
                Err(ErrorCode::UnexpectedEof) // Simulate parse error
            }
        }
    }

    let mut reader = TestReader { data: vec![b'h', b'e', b'l', b'l', b'o', b'"'], index: 0 };
    let mut scratch = Vec::new();

    let result: Result<String> = reader.parse_str_bytes(&mut scratch, true, |_, bytes| {
        String::from_utf8(bytes.to_vec()).map_err(|_| ErrorCode::Utf8Error)
    });

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");
}

#[test]
#[should_panic]
fn test_parse_str_bytes_control_character() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(ErrorCode::UnexpectedEof) // Simulate EOF
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Ok(()) // Simulate successful escape handling
        }
    }

    let mut reader = TestReader { data: vec![b'h', b'e', b'l', b'l', b'o', b'\x01', b'\"'], index: 0 };
    let mut scratch = Vec::new();

    let _result: Result<String> = reader.parse_str_bytes(&mut scratch, true, |_, bytes| {
        String::from_utf8(bytes.to_vec()).map_err(|_| ErrorCode::Utf8Error)
    });
}

#[test]
fn test_parse_str_bytes_escape_error() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn next_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                let val = self.data[self.index];
                self.index += 1;
                Ok(val)
            } else {
                Err(ErrorCode::UnexpectedEof) // Simulate EOF
            }
        }

        fn parse_escape(&mut self, _validate: bool, _scratch: &mut Vec<u8>) -> Result<()> {
            Err(ErrorCode::InvalidEscape) // Simulate parsing error
        }
    }

    let mut reader = TestReader { data: vec![b'h', b'e', b'l', b'l', b'o', b'\\', b'\"'], index: 0 };
    let mut scratch = Vec::new();

    let result: Result<String> = reader.parse_str_bytes(&mut scratch, true, |_, bytes| {
        String::from_utf8(bytes.to_vec()).map_err(|_| ErrorCode::Utf8Error)
    });

    assert!(result.is_err());
}

