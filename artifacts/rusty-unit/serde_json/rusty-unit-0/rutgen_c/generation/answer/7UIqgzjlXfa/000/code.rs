// Answer 0

#[test]
fn test_serializer_new() {
    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { data: Vec::new() };
    let serializer = Serializer::new(writer);
    
    // Verify that we can create a Serializer instance
    assert!(serializer.writer.is_some()); // Replace with proper assertion based on real implementation
}

#[test]
fn test_serializer_with_formatter() {
    struct MockFormatter;

    struct MockWriter {
        data: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let writer = MockWriter { data: Vec::new() };
    let formatter = MockFormatter;
    let serializer = Serializer::with_formatter(writer, formatter);
    
    // Verify that we can create a Serializer instance with a custom formatter
    assert!(serializer.writer.is_some()); // Replace with proper assertion based on real implementation
}

