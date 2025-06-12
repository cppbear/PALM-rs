// Answer 0

#[test]
fn test_decoder_reader_debug_fmt() {
    struct MockEngine;
    struct MockConfig;
    struct MockDecodeEstimate;

    impl crate::Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<crate::DecodeMetadata, crate::DecodeSliceError> {
            Ok(crate::DecodeMetadata { len: 0 }) // Assuming DecodeMetadata has only 'len'
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let mut buffer = [0u8; crate::BUF_SIZE];
    let reader = crate::DecoderReader {
        engine: &engine,
        inner: std::io::empty(),
        b64_buffer: buffer,
        b64_offset: 0,
        b64_len: 0,
        decoded_chunk_buffer: [0u8; crate::DECODED_CHUNK_SIZE],
        decoded_offset: 0,
        decoded_len: 0,
        input_consumed_len: 0,
        padding_offset: None,
    };

    let output = format!("{:?}", reader);

    assert!(output.contains("b64_offset: 0"));
    assert!(output.contains("b64_len: 0"));
    assert!(output.contains("decoded_chunk_buffer: [0, 0, 0]"));
    assert!(output.contains("decoded_offset: 0"));
    assert!(output.contains("decoded_len: 0"));
    assert!(output.contains("input_consumed_len: 0"));
    assert!(output.contains("padding_offset: None"));
}

#[test]
fn test_decoder_reader_with_padding_offset() {
    struct MockEngine;
    struct MockConfig;
    struct MockDecodeEstimate;

    impl crate::Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<crate::DecodeMetadata, crate::DecodeSliceError> {
            Ok(crate::DecodeMetadata { len: 0 }) // Assuming DecodeMetadata has only 'len'
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let mut buffer = [0u8; crate::BUF_SIZE];
    let reader = crate::DecoderReader {
        engine: &engine,
        inner: std::io::empty(),
        b64_buffer: buffer,
        b64_offset: 0,
        b64_len: 0,
        decoded_chunk_buffer: [0u8; crate::DECODED_CHUNK_SIZE],
        decoded_offset: 0,
        decoded_len: 0,
        input_consumed_len: 0,
        padding_offset: Some(42),
    };

    let output = format!("{:?}", reader);

    assert!(output.contains("padding_offset: Some(42)"));
}

