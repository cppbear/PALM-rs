// Answer 0

#[test]
fn test_get_ref_capacity() {
    use bytes::BufMut;
    use bytes::BytesMut;

    struct Writer {
        buf: BytesMut,
    }

    impl Writer {
        fn new(capacity: usize) -> Self {
            Writer {
                buf: BytesMut::with_capacity(capacity),
            }
        }

        fn get_ref(&self) -> &BytesMut {
            &self.buf
        }
    }

    let writer = Writer::new(1024);
    assert_eq!(1024, writer.get_ref().capacity());
}

#[test]
fn test_get_ref_empty_capacity() {
    use bytes::BufMut;
    use bytes::BytesMut;

    struct Writer {
        buf: BytesMut,
    }

    impl Writer {
        fn new(capacity: usize) -> Self {
            Writer {
                buf: BytesMut::with_capacity(capacity),
            }
        }

        fn get_ref(&self) -> &BytesMut {
            &self.buf
        }
    }

    let writer = Writer::new(0);
    assert_eq!(0, writer.get_ref().capacity());
}

