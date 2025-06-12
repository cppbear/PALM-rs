// Answer 0

#[test]
fn test_pretty_serializer_with_writer() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { buffer: Vec::new() };
    
    let serializer: serde_json::Serializer<MockWriter, serde_json::PrettyFormatter> = 
        serde_json::Serializer::pretty(writer);

    // Validate that the buffer is still empty, as nothing has been written yet
    assert!(serializer.writer.buffer.is_empty());
}

#[test]
fn test_pretty_formatter_creation() {
    let formatter = serde_json::PrettyFormatter::new();
    
    assert_eq!(formatter.current_indent, 0);
    assert!(!formatter.has_value);
    assert_eq!(formatter.indent, b"  ");
} 

#[test]
fn test_pretty_formatter_with_indent() {
    let indent = b"    "; // Four spaces
    let formatter = serde_json::PrettyFormatter::with_indent(indent);
    
    assert_eq!(formatter.current_indent, 0);
    assert!(!formatter.has_value);
    assert_eq!(formatter.indent, indent);
}

