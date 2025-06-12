// Answer 0

#[test]
fn test_encoder_writer_new() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct DummyWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for DummyWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let writer = DummyWriter { buffer: Vec::new() };
    let encoder_writer = EncoderWriter::new(writer, &engine);

    assert!(encoder_writer.delegate.is_some());
    assert_eq!(encoder_writer.extra_input_occupied_len, 0);
    assert_eq!(encoder_writer.output_occupied_len, 0);
    assert_eq!(encoder_writer.panicked, false);
} 

#[test]
fn test_encoder_writer_initialization() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct DummyWriter;

    impl io::Write for DummyWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let writer = DummyWriter;
    let encoder_writer = EncoderWriter::new(writer, &engine);

    assert_eq!(encoder_writer.extra_input, [0; MIN_ENCODE_CHUNK_SIZE]);
    assert_eq!(encoder_writer.output, [0; BUF_SIZE]);
}

