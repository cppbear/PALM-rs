// Answer 0

#[test]
fn test_write_all_encoded_output_interruption_error() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Err(DecodeSliceError) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter {
        buffer: Vec<u8>,
        interrupt_after: usize,
        write_count: usize,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_count += 1;
            if self.write_count == self.interrupt_after {
                return Err(io::Error::new(ErrorKind::Interrupted, "interrupted"));
            }
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let mut mock_writer = MockWriter { buffer: Vec::new(), interrupt_after: 2, write_count: 0 };
    let mut encoder_writer = EncoderWriter::new(mock_writer, &engine);
    
    encoder_writer.output_occupied_len = 3; // Set to a value > 0
    encoder_writer.output[..3].copy_from_slice(b"abc");
    
    let _ = encoder_writer.write_all_encoded_output(); // Expect an interruption error handled
}

#[test]
fn test_write_all_encoded_output_permanent_error() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Err(DecodeSliceError) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "permanent error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let mut mock_writer = MockWriter { buffer: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(mock_writer, &engine);
    
    encoder_writer.output_occupied_len = 3; // Set to a value > 0
    encoder_writer.output[..3].copy_from_slice(b"xyz");
    
    let err = encoder_writer.write_all_encoded_output(); // Expect the permanent error to propagate
}

#[test]
fn test_write_all_encoded_output_successful_write() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { Err(DecodeSliceError) }
        fn config(&self) -> &Self::Config { &() }
    }

    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let mut mock_writer = MockWriter { buffer: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(mock_writer, &engine);
    
    encoder_writer.output_occupied_len = 5; // Set to a value > 0
    encoder_writer.output[..5].copy_from_slice(b"hello");
    
    let _ = encoder_writer.write_all_encoded_output(); // Expect successful write without error
    assert_eq!(encoder_writer.output_occupied_len, 0); // After successful write, should be 0
}

