// Answer 0

#[test]
fn test_begin_object_success() {
    use std::io::{self, Cursor};

    struct TestWriter {
        inner: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                inner: Cursor::new(vec![]),
            }
        }
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.inner.flush()
        }
    }

    struct Serializer {
        current_indent: usize,
        has_value: bool,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                current_indent: 0,
                has_value: false,
            }
        }

        fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.current_indent += 1;
            self.has_value = false;
            writer.write_all(b"{")
        }
    }

    let mut serializer = Serializer::new();
    let mut writer = TestWriter::new();

    let result = serializer.begin_object(&mut writer);

    assert!(result.is_ok());
    assert_eq!(writer.inner.get_ref(), &b"{"[..]);
}

#[test]
fn test_begin_object_increment_indent() {
    struct TestWriter {
        inner: Cursor<Vec<u8>>,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                inner: Cursor::new(vec![]),
            }
        }
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.inner.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.inner.flush()
        }
    }

    struct Serializer {
        current_indent: usize,
        has_value: bool,
    }

    impl Serializer {
        fn new() -> Self {
            Self {
                current_indent: 0,
                has_value: false,
            }
        }

        fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            self.current_indent += 1;
            self.has_value = false;
            writer.write_all(b"{")
        }
    }

    let mut serializer = Serializer::new();
    let mut writer = TestWriter::new();

    let _ = serializer.begin_object(&mut writer);
    assert_eq!(serializer.current_indent, 1);
}

