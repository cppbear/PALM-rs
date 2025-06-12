// Answer 0

#[test]
fn test_begin_array_success() {
    use std::io::Cursor;
    use std::io::Write;

    struct TestSerializer {
        current_indent: usize,
        has_value: bool,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                current_indent: 0,
                has_value: false,
            }
        }

        fn begin_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            self.current_indent += 1;
            self.has_value = false;
            writer.write_all(b"[")
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut serializer = TestSerializer::new();
    let result = serializer.begin_array(&mut writer);

    assert!(result.is_ok());
    assert_eq!(writer.get_ref().as_slice(), b"[");
}

#[test]
fn test_begin_array_multiple_calls() {
    use std::io::Cursor;
    use std::io::Write;

    struct TestSerializer {
        current_indent: usize,
        has_value: bool,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                current_indent: 0,
                has_value: false,
            }
        }

        fn begin_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            self.current_indent += 1;
            self.has_value = false;
            writer.write_all(b"[")
        }
    }

    let mut writer = Cursor::new(Vec::new());
    let mut serializer = TestSerializer::new();

    for _ in 0..3 {
        let result = serializer.begin_array(&mut writer);
        assert!(result.is_ok());
        assert_eq!(writer.get_ref().as_slice(), b"[");
    }
}

#[test]
#[should_panic]
fn test_begin_array_panic_on_failed_write() {
    use std::io::Cursor;
    use std::io::Write;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct TestSerializer {
        current_indent: usize,
        has_value: bool,
    }

    impl TestSerializer {
        fn new() -> Self {
            TestSerializer {
                current_indent: 0,
                has_value: false,
            }
        }

        fn begin_array<W>(&mut self, writer: &mut W) -> std::io::Result<()>
        where
            W: ?Sized + std::io::Write,
        {
            self.current_indent += 1;
            self.has_value = false;
            writer.write_all(b"[")?;
            Ok(())
        }
    }

    let mut writer = FailingWriter;
    let mut serializer = TestSerializer::new();
    let _ = serializer.begin_array(&mut writer);
}

