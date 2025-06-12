// Answer 0

#[test]
fn test_peek_or_eof_ok() {
    use std::io::{Cursor, Read};
    
    struct MockReader {
        data: Cursor<Vec<u8>>,
    }

    impl<'de> Read<'de> for MockReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            let mut buf = [0; 1];
            let result = self.data.read(&mut buf);
            match result {
                Ok(1) => Ok(Some(buf[0])),
                Ok(_) => Ok(None),
                Err(e) => Err(e.into()),
            }
        }
    }

    let mut reader = MockReader {
        data: Cursor::new(vec![42]),
    };

    assert_eq!(peek_or_eof(&mut reader), Ok(42));
}

#[test]
fn test_peek_or_eof_eof_error() {
    use std::io::{Cursor, Read};

    struct MockReader {
        data: Cursor<Vec<u8>>,
    }

    impl<'de> Read<'de> for MockReader {
        fn peek(&mut self) -> Result<Option<u8>> {
            let mut buf = [0; 1];
            let result = self.data.read(&mut buf);
            match result {
                Ok(0) => Ok(None), // End of data
                Ok(1) => Ok(Some(buf[0])),
                Err(e) => Err(e.into()),
            }
        }
    }

    let mut reader = MockReader {
        data: Cursor::new(vec![]), // Empty data should trigger EOF
    };

    assert!(peek_or_eof(&mut reader).is_err());
}

