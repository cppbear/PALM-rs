// Answer 0

#[test]
fn test_serialize_bool_true() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"")
        }

        fn write_bool(&mut self, writer: &mut dyn io::Write, value: bool) -> Result<()> {
            if value {
                writer.write(b"true")
            } else {
                writer.write(b"false")
            }
        }

        fn end_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"")
        }
    }

    struct Serializer<W, F> {
        writer: W,
        formatter: F,
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let result = serializer.serialize_bool(true);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\"true\"");
}

#[test]
fn test_serialize_bool_false() {
    struct MockWriter {
        output: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"")
        }

        fn write_bool(&mut self, writer: &mut dyn io::Write, value: bool) -> Result<()> {
            if value {
                writer.write(b"true")
            } else {
                writer.write(b"false")
            }
        }

        fn end_string(&mut self, writer: &mut dyn io::Write) -> Result<()> {
            writer.write(b"\"")
        }
    }

    struct Serializer<W, F> {
        writer: W,
        formatter: F,
    }

    let mut writer = MockWriter { output: String::new() };
    let mut formatter = MockFormatter;
    let serializer = Serializer {
        writer: &mut writer,
        formatter,
    };

    let result = serializer.serialize_bool(false);
    assert!(result.is_ok());
    assert_eq!(writer.output, "\"false\"");
}

