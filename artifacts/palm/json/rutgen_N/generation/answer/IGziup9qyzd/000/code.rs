// Answer 0

#[test]
fn test_pretty_serializer_creation() {
    use serde_json::Serializer;
    use serde_json::ser::{PrettyFormatter, Serializer as JsonSerializer};
    
    struct TestWriter {
        buffer: Vec<u8>,
    }
    
    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }
        
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    
    let mut writer = TestWriter { buffer: vec![] };
    let serializer: JsonSerializer<TestWriter> = Serializer::pretty(writer);
    
    assert!(serializer.is_some());
}

