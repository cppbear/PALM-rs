// Answer 0

#[test]
fn test_serialize_unit_variant() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            MockWriter { buffer: Vec::new() }
        }
        
        fn into_bytes(self) -> Vec<u8> {
            self.buffer
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> Result<()> {
            self.write(buf)?;
            Ok(())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter for MockFormatter {
        fn begin_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn end_object(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn begin_object_key(&mut self, _writer: &mut dyn io::Write, _should_escape: bool) -> Result<()> {
            Ok(())
        }
        
        fn end_object_key(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }
        
        fn begin_object_value(&mut self, _writer: &mut dyn io::Write) -> Result<()> {
            Ok(())
        }

        fn serialize_str(&mut self, _writer: &mut dyn io::Write, _value: &str) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter;

    let mut serializer = Serializer {
        writer,
        formatter,
    };

    let result = serializer.serialize_unit_variant("test_name", 0, "variant_name");

    assert!(result.is_ok());
}

