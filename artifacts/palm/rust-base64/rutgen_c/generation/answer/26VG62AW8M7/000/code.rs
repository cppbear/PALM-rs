// Answer 0

#[test]
fn test_base64_display_new() {
    struct MockConfig;
    struct MockDecodeEstimate;
    struct MockEngine;

    impl Config for MockConfig {}
    impl DecodeEstimate for MockDecodeEstimate {}

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let encoded_length = input.len(); // simple pass-through for testing
            if encoded_length <= output.len() {
                output[..encoded_length].copy_from_slice(input);
            }
            encoded_length
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            // Return Ok for testing purposes
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }

        #[cfg(any(feature = "alloc", test))]
        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            String::from_utf8_lossy(input.as_ref()).to_string()
        }

        #[cfg(any(feature = "alloc", test))]
        fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
            output_buf.push_str(&self.encode(input));
        }

        #[inline]
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let bytes = input.as_ref();
            let length = self.internal_encode(bytes, output_buf);
            Ok(length)
        }

        #[cfg(any(feature = "alloc", test))]
        fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError> {
            Ok(input.as_ref().to_vec())
        }

        #[cfg(any(feature = "alloc", test))]
        fn decode_vec<T: AsRef<[u8]>>(&self, input: T, buffer: &mut Vec<u8>) -> Result<(), DecodeError> {
            buffer.extend_from_slice(input.as_ref());
            Ok(())
        }

        fn decode_slice<T: AsRef<[u8]>>(&self, input: T, output: &mut [u8]) -> Result<usize, DecodeSliceError> {
            let bytes = input.as_ref();
            let length = bytes.len();
            output[..length].copy_from_slice(bytes);
            Ok(length)
        }

        fn decode_slice_unchecked<T: AsRef<[u8]>>(&self, input: T, output: &mut [u8]) -> Result<usize, DecodeError> {
            self.decode_slice(input, output)
        }
    }

    let bytes: &[u8] = b"Hello, world!";
    let engine = MockEngine;

    let display = Base64Display::new(bytes, &engine);
    assert_eq!(display.bytes, bytes);
}

