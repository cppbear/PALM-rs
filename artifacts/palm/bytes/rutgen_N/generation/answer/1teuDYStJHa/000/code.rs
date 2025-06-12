// Answer 0

#[test]
fn test_into_inner() {
    use bytes::Buf;
    use std::io::Cursor;
    use std::io;

    struct TestReader {
        buf: Cursor<&'static [u8]>,
    }

    impl TestReader {
        fn new(data: &'static [u8]) -> Self {
            TestReader {
                buf: Cursor::new(data),
            }
        }

        fn into_inner(self) -> Cursor<&'static [u8]> {
            self.buf
        }

        fn remaining(&self) -> usize {
            self.buf.get_ref().len() - self.buf.position() as usize
        }
    }

    let mut buf = TestReader::new(b"hello world");
    let mut dst = vec![];

    io::copy(&mut buf.buf, &mut dst).unwrap();

    let buf = buf.into_inner();
    assert_eq!(0, buf.remaining());
}

#[test]
fn test_into_inner_empty() {
    use bytes::Buf;
    use std::io::Cursor;
    use std::io;

    struct TestReader {
        buf: Cursor<&'static [u8]>,
    }

    impl TestReader {
        fn new(data: &'static [u8]) -> Self {
            TestReader {
                buf: Cursor::new(data),
            }
        }

        fn into_inner(self) -> Cursor<&'static [u8]> {
            self.buf
        }

        fn remaining(&self) -> usize {
            self.buf.get_ref().len() - self.buf.position() as usize
        }
    }

    let mut buf = TestReader::new(b"");
    let mut dst = vec![];

    io::copy(&mut buf.buf, &mut dst).unwrap();

    let buf = buf.into_inner();
    assert_eq!(0, buf.remaining());
}

