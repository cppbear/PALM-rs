// Answer 0

#[test]
fn test_encoder_writer_new() {
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockEngine;

    let writer = MockWriter { buffer: Vec::new() };
    let engine = MockEngine;
    
    let encoder = base64::new(writer, &engine);
    
    assert!(encoder.delegate.is_some());
    assert_eq!(encoder.extra_input.len(), base64::MIN_ENCODE_CHUNK_SIZE);
    assert_eq!(encoder.output.len(), base64::BUF_SIZE);
    assert_eq!(encoder.output_occupied_len, 0);
    assert!(!encoder.panicked);
}

