// Answer 0

#[test]
fn test_get_mut_non_empty_buf() {
    use bytes::BufMut;
    use bytes::BytesMut;

    struct TestWriter {
        buf: BytesMut,
    }

    impl TestWriter {
        fn new() -> Self {
            TestWriter {
                buf: BytesMut::with_capacity(128),
            }
        }
        
        fn get_mut(&mut self) -> &mut BytesMut {
            &mut self.buf
        }
    }

    let mut writer = TestWriter::new();
    let mut buf_mut = writer.get_mut();
    buf_mut.reserve(1024);
    assert_eq!(1024, buf_mut.capacity());
}

#[test]
fn test_get_mut_empty_buf() {
    use bytes::BufMut;
    use bytes::BytesMut;

    struct EmptyWriter {
        buf: BytesMut,
    }

    impl EmptyWriter {
        fn new() -> Self {
            EmptyWriter {
                buf: BytesMut::new(),
            }
        }

        fn get_mut(&mut self) -> &mut BytesMut {
            &mut self.buf
        }
    }

    let mut writer = EmptyWriter::new();
    let mut buf_mut = writer.get_mut();
    buf_mut.reserve(512);
    assert_eq!(512, buf_mut.capacity());
}

#[should_panic]
#[test]
fn test_get_mut_panic_on_accessing_buf_after_drop() {
    use bytes::BufMut;
    use bytes::BytesMut;

    struct PanicWriter {
        buf: BytesMut,
    }

    impl PanicWriter {
        fn new() -> Self {
            PanicWriter {
                buf: BytesMut::with_capacity(256),
            }
        }

        fn get_mut(&mut self) -> &mut BytesMut {
            &mut self.buf
        }
    }

    let mut writer = PanicWriter::new();
    let _buf_mut = writer.get_mut(); // get mutable reference
    drop(writer); // drop the writer
    let _should_panic_buf_mut = writer.get_mut(); // this should panic
}

