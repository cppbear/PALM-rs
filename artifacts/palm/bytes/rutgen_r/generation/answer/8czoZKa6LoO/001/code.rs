// Answer 0

#[test]
fn test_into_inner_empty_buffer() {
    use bytes::BufMut;
    use std::io::Cursor;

    struct TestWriter {
        buf: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                buf: Cursor::new(Vec::new()),
            }
        }

        fn into_inner(self) -> Vec<u8> {
            self.buf.into_inner()
        }
    }

    let writer = TestWriter::new();
    let buf = writer.into_inner();
    assert_eq!(buf, Vec::<u8>::new());
}

#[test]
fn test_into_inner_non_empty_buffer() {
    use bytes::BufMut;
    use std::io;

    struct TestWriter {
        buf: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new(vec: Vec<u8>) -> Self {
            TestWriter {
                buf: Cursor::new(vec),
            }
        }

        fn into_inner(self) -> Vec<u8> {
            self.buf.into_inner()
        }
    }

    let mut vec = Vec::new();
    let mut src = &b"hello world"[..];
    vec.extend_from_slice(&b"hello world"[..]);

    let writer = TestWriter::new(vec);
    let buf = writer.into_inner();
    assert_eq!(buf, b"hello world"[..]);
}

#[test]
#[should_panic]
fn test_into_inner_panic_condition() {
    use bytes::BufMut;

    struct TestWriter {
        buf: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new(vec: Vec<u8>) -> Self {
            TestWriter {
                buf: Cursor::new(vec),
            }
        }

        fn into_inner(self) -> Vec<u8> {
            if self.buf.get_ref().is_empty() {
                panic!("Buffer cannot be empty");
            }
            self.buf.into_inner()
        }
    }

    let writer = TestWriter::new(Vec::new()); // This will trigger a panic
    let _ = writer.into_inner();
}

