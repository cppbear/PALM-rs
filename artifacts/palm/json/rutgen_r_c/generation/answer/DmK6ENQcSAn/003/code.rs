// Answer 0

#[test]
fn test_collect_str_valid_case() {
    struct MockWriter {
        data: Vec<u8>,
        should_fail: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.should_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "write error")))
            } else {
                self.data.extend_from_slice(buf);
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        should_begin_fail: bool,
        should_end_fail: bool,
    }

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, writer: &mut W) -> Result<()> {
            if self.should_begin_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "begin_string fail")))
            } else {
                Ok(())
            }
        }

        fn end_string<W: io::Write>(&mut self, writer: &mut W) -> Result<()> {
            if self.should_end_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "end_string fail")))
            } else {
                Ok(())
            }
        }
    }

    let mut writer = MockWriter { data: Vec::new(), should_fail: false };
    let mut formatter = MockFormatter { should_begin_fail: false, should_end_fail: false };
    let serializer = &mut Serializer { writer: &mut writer, formatter: &mut formatter };

    assert!(serializer.collect_str(&"Test").is_ok());
    assert_eq!(writer.data, b"Test");
}

#[test]
#[should_panic]
fn test_collect_str_begin_string_fail() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter {
        should_begin_fail: bool,
    }

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _writer: &mut W) -> Result<()> {
            if self.should_begin_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "begin_string fail")))
            } else {
                Ok(())
            }
        }

        fn end_string<W: io::Write>(&mut self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let formatter = &mut MockFormatter { should_begin_fail: true };
    let serializer = &mut Serializer { writer: &mut writer, formatter };

    let _ = serializer.collect_str(&"Test");
}

#[test]
#[should_panic]
fn test_collect_str_write_str_fail() {
    struct MockWriter {
        data: Vec<u8>,
        should_fail: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.should_fail {
                Err(Error::io(io::Error::new(io::ErrorKind::Other, "write error")))
            } else {
                self.data.extend_from_slice(buf);
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _: &mut W) -> Result<()> { Ok(()) }
        fn end_string<W: io::Write>(&mut self, _: &mut W) -> Result<()> { Ok(()) }
    }

    let mut writer = MockWriter { data: Vec::new(), should_fail: true };
    let formatter = &mut MockFormatter;
    let serializer = &mut Serializer { writer: &mut writer, formatter };

    let _ = serializer.collect_str(&"Test");
}

#[test]
fn test_collect_str_empty_string() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&mut self, _: &mut W) -> Result<()> { Ok(()) }
        fn end_string<W: io::Write>(&mut self, _: &mut W) -> Result<()> { Ok(()) }
    }

    let mut writer = MockWriter { data: Vec::new() };
    let formatter = &mut MockFormatter;
    let serializer = &mut Serializer { writer: &mut writer, formatter };

    assert!(serializer.collect_str(&"").is_ok());
    assert_eq!(writer.data, b"");
}

