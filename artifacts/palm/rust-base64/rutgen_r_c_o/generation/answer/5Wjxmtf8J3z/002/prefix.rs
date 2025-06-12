// Answer 0

#[test]
fn test_decode_to_buf_basic_case() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> usize { input_len }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: usize,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..decode_estimate].copy_from_slice(input);
            Ok(DecodeMetadata { decoded_len: decode_estimate, padding_offset: None })
        }

        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let input_data = b"SGVsbG8sIFdvcmxkIQ=="; // Base64 for "Hello, World!"
    let mut reader = DecoderReader::new(&input_data[..], &engine);

    reader.b64_len = input_data.len();
    reader.b64_offset = BUF_SIZE - input_data.len();

    let mut output_buf = vec![0u8; 3]; // Allocate enough space for decoding
    let result = reader.decode_to_buf(input_data.len(), &mut output_buf);

    let _ = result;
}

#[test]
fn test_decode_to_buf_exact_fit() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> usize { input_len }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: usize,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..decode_estimate].copy_from_slice(input);
            Ok(DecodeMetadata { decoded_len: decode_estimate, padding_offset: None })
        }

        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let input_data = b"QmFzZTY0"; // Base64 for "Base64"
    let mut reader = DecoderReader::new(&input_data[..], &engine);

    reader.b64_len = input_data.len();
    reader.b64_offset = BUF_SIZE - input_data.len();

    let mut output_buf = vec![0u8; 4]; // Allocate space for the expected output
    let result = reader.decode_to_buf(input_data.len(), &mut output_buf);

    let _ = result;
}

#[test]
fn test_decode_to_buf_partial_decode() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> usize { input_len }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: usize,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..decode_estimate].copy_from_slice(input);
            Ok(DecodeMetadata { decoded_len: decode_estimate, padding_offset: None })
        }

        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let input_data = b"U29tZSBleGFtcGxl"; // Base64 for "Some example"
    let mut reader = DecoderReader::new(&input_data[..], &engine);

    reader.b64_len = input_data.len();
    reader.b64_offset = BUF_SIZE - input_data.len();

    let mut output_buf = vec![0u8; 5]; // Allocate space for partial output
    let result = reader.decode_to_buf(input_data.len(), &mut output_buf);

    let _ = result;
}

#[should_panic]
#[test]
fn test_decode_to_buf_panic_short_buffer() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> usize { input_len }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: usize,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..decode_estimate].copy_from_slice(input);
            Ok(DecodeMetadata { decoded_len: decode_estimate, padding_offset: None })
        }

        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let input_data = b"VGVzdCBzdHJpbmc="; // Base64 for "Test string"
    let mut reader = DecoderReader::new(&input_data[..], &engine);

    reader.b64_len = input_data.len();
    reader.b64_offset = BUF_SIZE - input_data.len();

    let mut output_buf = vec![0u8; 2]; // Intentionally small buffer
    let _ = reader.decode_to_buf(input_data.len(), &mut output_buf);
}

