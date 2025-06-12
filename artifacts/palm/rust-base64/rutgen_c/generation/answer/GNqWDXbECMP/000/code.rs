// Answer 0

#[test]
fn test_encoder_writer_debug_fmt() {
    struct MockEngine;

    impl Engine for MockEngine {
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
        ) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        output_occupied_len: 5,
        panicked: false,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", writer);
    assert!(result.is_ok());
    assert_eq!(output, "extra_input: [1, 2, 3] extra_input_occupied_len:3 output[..5]: [4, 5, 6, 7, 8] output_occupied_len: 5");
}

