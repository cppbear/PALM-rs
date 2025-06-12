// Answer 0

#[test]
fn test_read_with_buf_length_1() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..3].copy_from_slice(b"abc");
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buf = [0_u8; 1];
    let engine = MockEngine;
    let input = b"YWJj"; // base64 for "abc"
    let mut reader = DecoderReader::new(&input[..], &engine);
    reader.b64_len = 4;
    reader.b64_offset = 0;
    reader.decoded_len = 3; // simulate already having 3 decoded bytes.
    reader.decoded_offset = 0;

    let _ = reader.read(&mut buf);
}

#[test]
fn test_read_with_buf_length_2() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..3].copy_from_slice(b"abc");
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buf = [0_u8; 2];
    let engine = MockEngine;
    let input = b"YWJj"; // base64 for "abc"
    let mut reader = DecoderReader::new(&input[..], &engine);
    reader.b64_len = 4;
    reader.b64_offset = 0;
    reader.decoded_len = 3; // simulate already having 3 decoded bytes.
    reader.decoded_offset = 0;

    let _ = reader.read(&mut buf);
} 

#[test]
fn test_read_with_complete_chunk() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..3].copy_from_slice(b"abc");
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buf = [0_u8; 2];
    let engine = MockEngine;
    let input = b"YWJj"; // base64 for "abc"
    let mut reader = DecoderReader::new(&input[..], &engine);
    reader.b64_len = 4;
    reader.b64_offset = 0;
    reader.decoded_len = 2; // simulate already having 2 decoded bytes.
    reader.decoded_offset = 0;

    let _ = reader.read(&mut buf);
} 

#[test]
fn test_read_at_eof_with_partial_chunk() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output[..2].copy_from_slice(b"ab");
            Ok(DecodeMetadata { decoded_len: 2, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }
    
    let mut buf = [0_u8; 2];
    let engine = MockEngine;
    let input = b"YWJ"; // base64 for "ab" (incomplete)
    let mut reader = DecoderReader::new(&input[..], &engine);
    reader.b64_len = 3;
    reader.b64_offset = 0;
    reader.decoded_len = 0; // No decoded bytes available
    reader.decoded_offset = 0;

    let _ = reader.read(&mut buf);
} 

#[test]
fn test_read_with_full_buffer() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output.copy_from_slice(b"abcdef");
            Ok(DecodeMetadata { decoded_len: 6, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mut buf = [0_u8; 6];
    let engine = MockEngine;
    let input = b"QUJDREU="; // base64 for "ABCDE"
    let mut reader = DecoderReader::new(&input[..], &engine);
    reader.b64_len = 8; // Set to a full-length
    reader.b64_offset = 0;
    reader.decoded_len = 0; // No decoded bytes available
    reader.decoded_offset = 0;

    let _ = reader.read(&mut buf);
} 

