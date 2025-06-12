// Answer 0

#[test]
fn test_write_with_empty_src() {
    use std::io;
    use std::cmp;
    use bytes::BytesMut;

    struct Writer {
        buf: BytesMut,
    }

    impl Writer {
        fn new(size: usize) -> Self {
            Writer {
                buf: BytesMut::with_capacity(size),
            }
        }

        fn write(&mut self, src: &[u8]) -> io::Result<usize> {
            let n = cmp::min(self.buf.remaining_mut(), src.len());
            self.buf.put_slice(&src[..n]);
            Ok(n)
        }
    }

    let mut writer = Writer::new(10);
    let result = writer.write(&[]);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_write_with_exact_fit() {
    use std::io;
    use std::cmp;
    use bytes::BytesMut;

    struct Writer {
        buf: BytesMut,
    }

    impl Writer {
        fn new(size: usize) -> Self {
            Writer {
                buf: BytesMut::with_capacity(size),
            }
        }

        fn write(&mut self, src: &[u8]) -> io::Result<usize> {
            let n = cmp::min(self.buf.remaining_mut(), src.len());
            self.buf.put_slice(&src[..n]);
            Ok(n)
        }
    }

    let mut writer = Writer::new(5);
    let result = writer.write(&[1, 2, 3, 4, 5]);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_write_with_larger_src() {
    use std::io;
    use std::cmp;
    use bytes::BytesMut;

    struct Writer {
        buf: BytesMut,
    }

    impl Writer {
        fn new(size: usize) -> Self {
            Writer {
                buf: BytesMut::with_capacity(size),
            }
        }

        fn write(&mut self, src: &[u8]) -> io::Result<usize> {
            let n = cmp::min(self.buf.remaining_mut(), src.len());
            self.buf.put_slice(&src[..n]);
            Ok(n)
        }
    }

    let mut writer = Writer::new(3);
    let result = writer.write(&[1, 2, 3, 4, 5]);
    assert_eq!(result, Ok(3));
}

#[test]
fn test_write_with_non_exceeding_changes() {
    use std::io;
    use std::cmp;
    use bytes::BytesMut;

    struct Writer {
        buf: BytesMut,
    }

    impl Writer {
        fn new(size: usize) -> Self {
            Writer {
                buf: BytesMut::with_capacity(size),
            }
        }

        fn write(&mut self, src: &[u8]) -> io::Result<usize> {
            let n = cmp::min(self.buf.remaining_mut(), src.len());
            self.buf.put_slice(&src[..n]);
            Ok(n)
        }
    }

    let mut writer = Writer::new(6);
    let result = writer.write(&[1, 2]);
    assert_eq!(result, Ok(2));
}

