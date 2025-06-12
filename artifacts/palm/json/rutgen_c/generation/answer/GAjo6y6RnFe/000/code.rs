// Answer 0

#[test]
fn test_serialize_i128_positive() {
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
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_i128(&mut self, _writer: &mut dyn io::Write, value: i128) -> Result<()> {
            let str_value = value.to_string();
            _writer.write(str_value.as_bytes())?;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut serializer = Serializer {
        writer,
        formatter: MockFormatter,
    };
    let result = serializer.serialize_i128(123456789012345678901234567890i128);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.output).unwrap(), "123456789012345678901234567890");
}

#[test]
fn test_serialize_i128_negative() {
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
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn write_i128(&mut self, _writer: &mut dyn io::Write, value: i128) -> Result<()> {
            let str_value = value.to_string();
            _writer.write(str_value.as_bytes())?;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut serializer = Serializer {
        writer,
        formatter: MockFormatter,
    };
    let result = serializer.serialize_i128(-123456789012345678901234567890i128);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(serializer.writer.output).unwrap(), "-123456789012345678901234567890");
}

