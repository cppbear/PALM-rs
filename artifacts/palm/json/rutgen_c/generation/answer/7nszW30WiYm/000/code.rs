// Answer 0

#[test]
fn test_serialize_i32_success() {
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
            // Begin the string (could output a quote mark if needed)
            Ok(())
        }

        fn write_i32(&mut self, _writer: &mut dyn io::Write, value: i32) -> Result<()> {
            // Writing integer as bytes
            let bytes = value.to_string().as_bytes();
            _writer.write(bytes)?;
            Ok(())
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            // End the string (could output a closing quote mark if needed)
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer { writer, formatter };
    let result = serializer.serialize_i32(123);

    assert!(result.is_ok());
    assert_eq!(String::from_utf8(result.unwrap().output).unwrap(), "123");
}

#[test]
#[should_panic(expected = "key_must_be_a_string()")]
fn test_serialize_i32_fail() {
    struct MockWriterFail {
        output: Vec<u8>,
    }

    impl io::Write for MockWriterFail {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            // Simulate a write failure
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "write error")))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatterFail;

    impl Formatter for MockFormatterFail {
        fn begin_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn write_i32(&mut self, _writer: &mut dyn io::Write, _value: i32) -> Result<()> {
            // Simulating a failure in writing
            Err(Error::io(std::io::Error::new(std::io::ErrorKind::Other, "write failure")))
        }

        fn end_string(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriterFail { output: Vec::new() };
    let formatter = MockFormatterFail;
    let serializer = Serializer { writer, formatter };
    
    // This should panic
    let _ = serializer.serialize_i32(123);
}

