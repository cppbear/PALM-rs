// Answer 0

#[test]
fn test_serialize_f64_finite_value() {
    struct MockFormatter;
    struct MockWriter {
        output: Vec<u8>,
        write_error: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut MockWriter, _value: f64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: Vec::new(),
        write_error: false,
    };
    let mock_formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter: mock_formatter,
    };

    let result = serializer.serialize_f64(1.23);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_f64_non_finite_value() {
    struct MockFormatter;
    struct MockWriter {
        output: Vec<u8>,
        write_error: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut MockWriter, _value: f64) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: Vec::new(),
        write_error: false,
    };
    let mock_formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter: mock_formatter,
    };

    let result = serializer.serialize_f64(f64::NAN);
    assert!(result.is_err());
}

#[test]
fn test_serialize_f64_write_error() {
    struct MockFormatter;
    struct MockWriter {
        output: Vec<u8>,
        write_error: bool,
    }

    impl MockFormatter {
        fn begin_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_f64(&mut self, _writer: &mut MockWriter, _value: f64) -> Result<()> {
            if _writer.write_error {
                Err(Error)
            } else {
                Ok(())
            }
        }

        fn end_string(&mut self, _writer: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter {
        output: Vec::new(),
        write_error: true,
    };
    let mock_formatter = MockFormatter;
    let serializer = Serializer {
        writer,
        formatter: mock_formatter,
    };

    let result = serializer.serialize_f64(1.23);
    assert!(result.is_err());
}

