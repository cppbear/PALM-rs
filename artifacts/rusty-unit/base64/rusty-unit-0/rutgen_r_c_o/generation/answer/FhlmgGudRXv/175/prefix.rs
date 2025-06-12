// Answer 0

#[test]
fn test_read_non_empty_buf_partially_decoded() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            let len = input.len(); // just mock decoding behaviour
            output[..len / 4 * 3].copy_from_slice(&[0; 3]); // mock output
            Ok(DecodeMetadata { decoded_len: len / 4 * 3, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"U29tZSBkYXRh"; // base64 for "Some data"
    let reader = input_data.as_slice();
    let mut decoder = DecoderReader::new(reader, &engine);
    
    let mut buf = vec![0; 5]; // buffer with length > 0
    decoder.b64_offset = 0;
    decoder.b64_len = BASE64_CHUNK_SIZE; // maximum base64 chunk
    decoder.decoded_len = 2; // non-zero decoded length
    decoder.decoded_offset = 1; // offset into decoded buffer

    let _ = decoder.read(&mut buf);
}

#[test]
fn test_read_empty_buffer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: input.len() / 4 * 3, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"U29tZSBkYXRh";
    let reader = input_data.as_slice();
    let mut decoder = DecoderReader::new(reader, &engine);
    
    let mut buf: Vec<u8> = vec![]; // empty buffer
    
    let _ = decoder.read(&mut buf);
}

#[test]
fn test_read_full_buffer_after_decoding() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output.copy_from_slice(&[0; DECODED_CHUNK_SIZE]);
            Ok(DecodeMetadata { decoded_len: DECODED_CHUNK_SIZE, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"U29tZSBkYXRh"; // base64 representation
    let reader = input_data.as_slice();
    let mut decoder = DecoderReader::new(reader, &engine);
    
    let mut buf = vec![0; 3]; // sufficient buffer to receive decoded data
    decoder.b64_offset = 0;
    decoder.b64_len = BASE64_CHUNK_SIZE; // full
    decoder.decoded_len = DECODED_CHUNK_SIZE; // already decoded chunk
    decoder.decoded_offset = 0; // ready to flush

    let _ = decoder.read(&mut buf);
}

#[test]
fn test_read_at_eof_with_partial_data() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len / 4 * 3 }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output.copy_from_slice(&[0; 2]); // mock two bytes decoded
            Ok(DecodeMetadata { decoded_len: 2, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"U29tZ"; // partial base64
    let reader = input_data.as_slice();
    let mut decoder = DecoderReader::new(reader, &engine);
    
    let mut buf = vec![0; 4]; // buffer to read into
    decoder.b64_offset = 0;
    decoder.b64_len = 4; // full base64
    decoder.decoded_len = 0; // nothing decoded yet
    decoder.decoded_offset = 0; // reset offset
    let _ = decoder.read(&mut buf);
}

