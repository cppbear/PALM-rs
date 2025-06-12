// Answer 0

#[test]
fn test_serialize_i128_success() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_i128<W: io::Write>(&self, _writer: &mut W, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_i128(42);

    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_i128_begin_string_error() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "begin_string error")))
        }

        fn write_i128<W: io::Write>(&self, _writer: &mut W, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_i128(42);
}

#[test]
#[should_panic]
fn test_serialize_i128_write_i128_error() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_i128<W: io::Write>(&self, _writer: &mut W, _value: i128) -> Result<()> {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "write_i128 error")))
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_i128(42);
}

#[test]
#[should_panic]
fn test_serialize_i128_end_string_error() {
    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_i128<W: io::Write>(&self, _writer: &mut W, _value: i128) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "end_string error")))
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_i128(42);
}

