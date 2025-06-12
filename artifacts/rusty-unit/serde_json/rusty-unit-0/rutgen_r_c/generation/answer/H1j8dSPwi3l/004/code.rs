// Answer 0

#[test]
fn test_parse_escape_backslash() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            // This is a simplified mock for testing
            Ok(0x0074) // returning 't'
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b't']);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\t']);
}

#[test]
fn test_parse_escape_double_quote() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0x0072) // returning 'r'
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'"']);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'"']);
}

#[test]
fn test_parse_escape_backward_slash() {
    struct TestReader {
        data: Vec<u8>,
        index: usize,
    }

    impl TestReader {
        fn new(data: Vec<u8>) -> Self {
            TestReader { data, index: 0 }
        }
    }

    impl<'de> Read<'de> for TestReader {
        fn next(&mut self) -> Result<Option<u8>> {
            if self.index < self.data.len() {
                let byte = self.data[self.index];
                self.index += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        }

        fn decode_hex_escape(&mut self) -> Result<i16> {
            Ok(0x0062) // returning 'b'
        }

        fn discard(&mut self) {}

        fn peek_or_eof(&mut self) -> Result<u8> {
            if self.index < self.data.len() {
                Ok(self.data[self.index])
            } else {
                Err(Error::from(ErrorCode::EofWhileParsingString))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestReader::new(vec![b'\\', b'b']);
    let result = parse_escape(&mut reader, false, &mut scratch);
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\x08']);
}

