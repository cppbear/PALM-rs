// Answer 0

#[test]
fn test_write_final_leftovers_delegate_none() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = cmp::min(input.len(), output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let buffer: Vec<u8> = vec![0; BUF_SIZE];
    let mut encoder = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false
    };

    let result = encoder.write_final_leftovers();

    assert!(result.is_ok());
}

