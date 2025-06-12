// Answer 0

#[test]
fn test_decoder_reader_new() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { ..Default::default() }) // Placeholder for DecodeMetadata
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let reader = std::io::Cursor::new("dummy data");
    let engine = DummyEngine;
    let decoder = DecoderReader::new(reader, &engine);

    assert_eq!(decoder.b64_offset, 0);
    assert_eq!(decoder.b64_len, 0);
    assert_eq!(decoder.decoded_offset, 0);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.input_consumed_len, 0);
    assert!(decoder.padding_offset.is_none());
    assert_eq!(decoder.b64_buffer.len(), BUF_SIZE);
    assert_eq!(decoder.decoded_chunk_buffer.len(), DECODED_CHUNK_SIZE);
}

