// Answer 0

#[test]
fn test_serialize_u32_success() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { output: Vec::new() }
        }
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
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

    impl MockFormatter {
        fn write_u32(&self, writer: &mut dyn io::Write, value: u32) -> Result<()> {
        let bytes = value.to_le_bytes();
        writer.write_all(&bytes)
        }
    }

    let writer = MockWriter::new();
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let result = serializer.serialize_u32(42);
    assert!(result.is_ok());
}

#[test]
#[should_panic]
fn test_serialize_u32_panic_on_write() {
    struct PanicWriter;

    impl io::Write for PanicWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            panic!("Writing failed");
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

    impl MockFormatter {
        fn write_u32(&self, writer: &mut dyn io::Write, value: u32) -> Result<()> {
            let bytes = value.to_le_bytes();
            writer.write_all(&bytes)
        }
    }

    let writer = PanicWriter;
    let formatter = MockFormatter;
    let mut serializer = Serializer { writer, formatter };

    let _ = serializer.serialize_u32(100); 
}

