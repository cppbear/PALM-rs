// Answer 0

#[test]
fn test_serialize_f32_finite() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f32(1.23);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_f32_nan() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let _result = serializer.serialize_f32(f32::NAN);
}

#[test]
fn test_serialize_f32_infinity() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f32(f32::INFINITY);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f32_negative() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<(), Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }

        fn write_f32(&mut self, _writer: &mut dyn io::Write, _value: f32) -> Result<(), Error> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<(), Error> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f32(-1.23);
    assert!(result.is_ok());
}

