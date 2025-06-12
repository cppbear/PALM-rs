// Answer 0

#[test]
fn test_parse_escape_invalid_escape_sequence() {
    use std::io::Cursor;
    use std::io::Read;
    use serde_json::ErrorCode;
    use serde_json::Result;

    struct MockReader {
        buffer: Vec<u8>,
        position: usize,
    }

    impl MockReader {
        fn new(buffer: Vec<u8>) -> Self {
            MockReader { buffer, position: 0 }
        }
    }

    impl<'de> Read<'de> for MockReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.position < self.buffer.len() {
                buf[0] = self.buffer[self.position];
                self.position += 1;
                Ok(1)
            } else {
                Ok(0)
            }
        }
    }

    let inputs = vec![b'x', b'\0', b'\xFE']; // Invalid escape sequence inputs
    let validate = false;
    let mut scratch = Vec::new();

    for &input in &inputs {
        let mut reader = MockReader::new(vec![b'\\', input]);
        let result = parse_escape(&mut reader, validate, &mut scratch);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code(), ErrorCode::InvalidEscape);
    }
}

