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
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::new())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mock_writer = Vec::new(); // A mock writer that will hold the output, using Vec<u8>

    let encoder_writer = EncoderWriter::new(mock_writer, &engine);

    assert!(encoder_writer.delegate.is_some()); // Check if delegate is set
    assert_eq!(encoder_writer.extra_input, [0u8; MIN_ENCODE_CHUNK_SIZE]); // Ensure extra_input is initialized correctly
    assert_eq!(encoder_writer.extra_input_occupied_len, 0); // Ensure it's empty
    assert_eq!(encoder_writer.output, [0u8; BUF_SIZE]); // Ensure output buffer is initialized correctly
    assert_eq!(encoder_writer.output_occupied_len, 0); // Ensure it starts empty
    assert!(!encoder_writer.panicked); // Ensure panicked flag is false
}

#[test]
fn test_encoder_writer_new_empty_delegate() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::new())
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let mock_writer: Vec<u8> = Vec::new(); // Empty Vec as a delegate
    
    let encoder_writer = EncoderWriter::new(mock_writer, &engine);

    assert!(encoder_writer.delegate.is_some());
    assert_eq!(encoder_writer.extra_input, [0u8; MIN_ENCODE_CHUNK_SIZE]);
    assert_eq!(encoder_writer.extra_input_occupied_len, 0);
    assert_eq!(encoder_writer.output, [0u8; BUF_SIZE]);
    assert_eq!(encoder_writer.output_occupied_len, 0);
    assert!(!encoder_writer.panicked);
}

