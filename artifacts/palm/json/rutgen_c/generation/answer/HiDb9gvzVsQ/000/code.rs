// Answer 0

#[test]
fn test_with_formatter() {
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

    struct CompactFormatter;

    let mock_writer = MockWriter { output: Vec::new() };
    let formatter = CompactFormatter;

    let serializer: Serializer<MockWriter, CompactFormatter> = Serializer::with_formatter(mock_writer, formatter);

    assert_eq!(serializer.writer.output.len(), 0); // Check that the output is initially empty
}

