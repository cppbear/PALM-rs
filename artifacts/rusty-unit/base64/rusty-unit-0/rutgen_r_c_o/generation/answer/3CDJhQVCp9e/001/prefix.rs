// Answer 0

#[test]
fn test_write_final_leftovers_delegate_is_none() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output.copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata)
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let delegate: Option<Vec<u8>> = None;

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate,
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 2,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write_final_leftovers();
    // Expected: Ok(())
}

