// Answer 0

#[test]
fn test_serialize_u128() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_u128(&mut self, _: &mut MockWriter, value: u128) -> Result<()> {
            self.output.extend(value.to_string().as_bytes());
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_u128(12345678901234567890123456789012345678);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"12345678901234567890123456789012345678");
}

#[test]
fn test_serialize_u128_empty() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl MockFormatter {
        fn begin_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }

        fn write_u128(&mut self, _: &mut MockWriter, _: u128) -> Result<()> {
            Ok(())
        }

        fn end_string(&mut self, _: &mut MockWriter) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_u128(0);
    assert!(result.is_ok());
    assert_eq!(serializer.writer.output, b"");
}

