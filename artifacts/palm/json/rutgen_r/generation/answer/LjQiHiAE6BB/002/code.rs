// Answer 0

#[test]
fn test_peek_or_eof_success() {
    struct MockReader<'a> {
        data: &'a [u8],
        position: usize,
    }

    impl<'de> Read<'de> for MockReader<'_> {
        fn peek(&mut self) -> Result<Option<u8>> {
            if self.position < self.data.len() {
                Ok(Some(self.data[self.position]))
            } else {
                Ok(None)
            }
        }
    }

    let mut reader = MockReader {
        data: &[1, 2, 3],
        position: 0,
    };

    assert_eq!(peek_or_eof(&mut reader), Ok(1));
}

#[test]
fn test_peek_or_eof_error() {
    struct ErrorReader;

    impl<'de> Read<'de> for ErrorReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            Err(ErrorCode::SomeError.into())
        }
    }

    let mut reader = ErrorReader;

    assert!(peek_or_eof(&mut reader).is_err());
}

#[test]
fn test_peek_or_eof_eof() {
    struct EmptyReader;

    impl<'de> Read<'de> for EmptyReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            Ok(None)
        }
    }

    let mut reader = EmptyReader;

    assert!(peek_or_eof(&mut reader).is_err());
}

