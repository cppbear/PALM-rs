// Answer 0

#[test]
fn test_parse_escape_with_valid_r_character() {
    use std::io::Cursor;

    struct MockReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read<'a> for MockReader<'a> {
        fn next_or_eof(&mut self) -> Result<u8, AnyError> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(AnyError::from(ErrorCode::UnexpectedEof))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader { data: &[b'\\', b'r'], position: 0 };

    let result = parse_escape(&mut reader, false, &mut scratch);

    assert_eq!(result, Ok(()));
    assert_eq!(scratch, vec![b'\r']);
}

#[test]
fn test_parse_escape_with_invalid_character() {
    use std::io::Cursor;

    struct MockReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read<'a> for MockReader<'a> {
        fn next_or_eof(&mut self) -> Result<u8, AnyError> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(AnyError::from(ErrorCode::UnexpectedEof))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader { data: &[b'\\', b'x'], position: 0 };

    let result = parse_escape(&mut reader, false, &mut scratch);

    assert!(result.is_err());
}

#[test]
fn test_parse_escape_with_eof_after_backslash() {
    use std::io::Cursor;

    struct MockReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'a> Read<'a> for MockReader<'a> {
        fn next_or_eof(&mut self) -> Result<u8, AnyError> {
            if self.position < self.data.len() {
                let byte = self.data[self.position];
                self.position += 1;
                Ok(byte)
            } else {
                Err(AnyError::from(ErrorCode::UnexpectedEof))
            }
        }
    }

    let mut scratch = Vec::new();
    let mut reader = MockReader { data: &[b'\\'], position: 0 };

    let result = parse_escape(&mut reader, false, &mut scratch);

    assert!(result.is_err());
}

