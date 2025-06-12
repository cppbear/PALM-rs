// Answer 0

#[test]
fn test_get_ref_with_borrowed_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
            }
        }
    }

    struct Reader<'a> {
        buf: &'a TestBuf,
    }

    impl<'a> Reader<'a> {
        fn new(buf: &'a TestBuf) -> Self {
            Self { buf }
        }

        fn get_ref(&self) -> &'a TestBuf {
            &self.buf
        }
    }

    let buf = TestBuf::new(b"hello world");
    let reader = Reader::new(&buf);

    assert_eq!(reader.get_ref().data, buf.data);
}

#[test]
fn test_get_ref_with_empty_buf() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl TestBuf {
        fn new(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
            }
        }
    }

    struct Reader<'a> {
        buf: &'a TestBuf,
    }

    impl<'a> Reader<'a> {
        fn new(buf: &'a TestBuf) -> Self {
            Self { buf }
        }

        fn get_ref(&self) -> &'a TestBuf {
            &self.buf
        }
    }

    let buf = TestBuf::new(b"");
    let reader = Reader::new(&buf);

    assert_eq!(reader.get_ref().data, buf.data);
}

