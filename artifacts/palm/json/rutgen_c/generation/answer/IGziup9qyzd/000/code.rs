// Answer 0

#[test]
fn test_pretty_serializer_new() {
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

    let writer = MockWriter { output: Vec::new() };
    let serializer = Serializer::pretty(writer);
    assert!(serializer.writer.output.is_empty()); // Check initial output state
}

#[test]
fn test_pretty_formatter_new() {
    let formatter = PrettyFormatter::new();
    assert_eq!(formatter.current_indent, 0);
    assert!(!formatter.has_value);
    assert_eq!(formatter.indent, b"  ");
}

