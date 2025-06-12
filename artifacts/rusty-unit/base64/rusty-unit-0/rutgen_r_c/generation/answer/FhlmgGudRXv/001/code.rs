// Answer 0

#[test]
fn test_read_empty_buffer() {
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
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let reader: &[u8] = &[];
    let mut decoder_reader = DecoderReader::new(reader, &engine);
    let mut buf = &mut []; // empty buffer

    let result = decoder_reader.read(&mut buf);

    assert_eq!(result, Ok(0));
}

