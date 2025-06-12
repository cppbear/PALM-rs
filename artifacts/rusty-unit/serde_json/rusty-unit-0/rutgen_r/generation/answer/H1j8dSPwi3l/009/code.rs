// Answer 0

fn test_parse_escape_valid_slash() {
    use std::io::Cursor;
    use serde_json::ErrorCode;
    use serde_json::Result;

    struct MockReader {
        bytes: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            MockReader { bytes, index: 0 }
        }
    }

    impl<'de> std::io::Read<'de> for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.index >= self.bytes.len() {
                return Ok(0); // EOF
            }
            let byte = self.bytes[self.index];
            self.index += 1;
            buf[0] = byte;
            Ok(1)
        }
    }

    let mut scratch = Vec::new();
    let mut mock_reader = MockReader::new(vec![b'\\', b'/']);
    
    let result: Result<()> = parse_escape(&mut mock_reader, true, &mut scratch);
    
    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'/']);
}

fn test_parse_escape_invalid_escape() {
    use std::io::Cursor;
    use serde_json::ErrorCode;
    use serde_json::Result;

    struct MockReader {
        bytes: Vec<u8>,
        index: usize,
    }

    impl MockReader {
        fn new(bytes: Vec<u8>) -> Self {
            MockReader { bytes, index: 0 }
        }
    }

    impl<'de> std::io::Read<'de> for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.index >= self.bytes.len() {
                return Ok(0); // EOF
            }
            let byte = self.bytes[self.index];
            self.index += 1;
            buf[0] = byte;
            Ok(1)
        }
    }

    let mut scratch = Vec::new();
    let mut mock_reader = MockReader::new(vec![b'\\', b'x']);
    
    let result: Result<()> = parse_escape(&mut mock_reader, true, &mut scratch);
    
    assert!(result.is_err());
    assert_eq!(scratch, Vec::new());
}

