// Answer 0

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_with_no_delegate() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { length: 0 })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = DummyEngine;
    let writer: Vec<u8> = Vec::new();
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.finish();
}

