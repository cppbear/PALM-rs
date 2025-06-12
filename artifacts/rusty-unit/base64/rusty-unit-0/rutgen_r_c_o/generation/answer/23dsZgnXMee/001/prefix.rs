// Answer 0

#[test]
fn test_encode_partial_chunk_with_padding_fails() {
    struct MockEngine;
    struct MockConfig;
    struct ErrorSink;

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = ();

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {}

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &MockConfig {}
        }
    }

    impl ErrorSink {
        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), ()> {
            Err(())
        }
    }

    let engine = MockEngine {};
    let encoder = ChunkedEncoder::new(&engine);
    let bytes: &[u8] = &[0u8; 1000];
    let mut sink = ErrorSink {};

    let _ = encoder.encode(bytes, &mut sink);
}

#[test]
fn test_encode_partial_chunk_with_padding_no_error() {
    struct MockEngine;
    struct MockConfig;
    struct ValidSink;

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = ();

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {}

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<(), ()> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &MockConfig {}
        }
    }

    impl ValidSink {
        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            // Successfully write the bytes
            Ok(())
        }
    }

    let engine = MockEngine {};
    let encoder = ChunkedEncoder::new(&engine);
    let bytes: &[u8] = &[0u8; 1000];
    let mut sink = ValidSink {};

    let _ = encoder.encode(bytes, &mut sink);
}

