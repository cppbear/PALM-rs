// Answer 0

#[test]
fn test_parse_escape_with_forward_slash() {
    use std::io::Cursor;
    use serde_json::error::ErrorCode;
    use serde_json::de::next_or_eof;

    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for TestRead {
        fn next_or_eof(&mut self) -> Result<u8, std::io::Error> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "End of input"))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestRead::new(vec![b'\\', b'/']); // simulate a backslash followed by a forward slash
    let result = parse_escape(&mut reader, false, &mut scratch);

    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'/', b'\\']); // Check that the forward slash is added correctly.
}

#[test]
fn test_parse_escape_with_valid_escape_sequence() {
    use std::io::Cursor;
    use serde_json::error::ErrorCode;

    struct TestRead {
        data: Vec<u8>,
        position: usize,
    }

    impl TestRead {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }
    }

    impl<'de> Read<'de> for TestRead {
        fn next_or_eof(&mut self) -> Result<u8, std::io::Error> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "End of input"))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = TestRead::new(vec![b'\\', b'b']); // simulate a backslash followed by 'b'
    let result = parse_escape(&mut reader, false, &mut scratch);

    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\x08']); // Check that the backspace character is added correctly.
}

