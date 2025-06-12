// Answer 0

#[test]
fn test_serialize_f64_finite_value() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_f64(&mut self, _: &mut dyn io::Write, value: f64) -> Result<()> {
            let s = value.to_string();
            let result = format!("{}{}", b'"' as char, s);
            let result_len = result.len();
            self.begin_string(&mut result.as_bytes()).unwrap(); // simulate writing
            Ok(())
        }
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: vec![] };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(3.14);
    assert!(result.is_ok());
}

#[test]
fn test_serialize_f64_infinite_value() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        fn write_f64(&mut self, _: &mut dyn io::Write, _: f64) -> Result<()> {
            Ok(())
        }
        fn end_string(&mut self, _: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: vec![] };
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let result = serializer.serialize_f64(f64::INFINITY);
    assert!(result.is_err());
}

