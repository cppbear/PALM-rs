// Answer 0

#[test]
fn test_fill_buf_with_valid_chunk() {
    use std::io;
    use bytes::Buf;
    use bytes::Bytes;

    struct TestReader {
        buf: Bytes,
    }

    impl TestReader {
        fn new() -> Self {
            TestReader {
                buf: Bytes::from_static(b"Test data"),
            }
        }

        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(self.buf.chunk())
        }
    }

    let mut reader = TestReader::new();
    let result = reader.fill_buf();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"Test data");
}

#[test]
fn test_fill_buf_with_empty_chunk() {
    use std::io;
    use bytes::Bytes;

    struct EmptyTestReader {
        buf: Bytes,
    }

    impl EmptyTestReader {
        fn new() -> Self {
            EmptyTestReader {
                buf: Bytes::from_static(b""),
            }
        }

        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            Ok(self.buf.chunk())
        }
    }

    let mut reader = EmptyTestReader::new();
    let result = reader.fill_buf();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), b"");
}

