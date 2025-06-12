// Answer 0

#[test]
fn test_serialize_f64_finite_value() {
    struct MockWriter {
        data: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            self.data.push_str(core::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write(b"\"")?;
            Ok(())
        }

        fn write_f64<W: io::Write>(&self, writer: &mut W, value: f64) -> Result<()> {
            writer.write(format!("{}", value).as_bytes())?;
            Ok(())
        }

        fn end_string<W: io::Write>(&self, writer: &mut W) -> Result<()> {
            writer.write(b"\"")?;
            Ok(())
        }
    }

    let mut writer = MockWriter { data: String::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(42.0);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_nan() {
    struct MockWriter {
        data: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            // Simulate failure by not writing anything
            Ok(0)
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_f64<W: io::Write>(&self, _writer: &mut W, _value: f64) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: String::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_infinite_value() {
    struct MockWriter {
        data: String,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> core::result::Result<usize, std::io::Error> {
            // Simulate failure by not writing anything
            Ok(0)
        }

        fn flush(&mut self) -> core::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }

        fn write_f64<W: io::Write>(&self, _writer: &mut W, _value: f64) -> Result<()> {
            Ok(())
        }

        fn end_string<W: io::Write>(&self, _writer: &mut W) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { data: String::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_err());
}

