// Answer 0

#[test]
fn test_decode_to_buf_b64_len_less_than_b64_len_to_decode() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { 
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) 
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    reader.b64_len = 3;
    let mut buf = [0u8; 2];
    let result = reader.decode_to_buf(4, &mut buf);
}

#[test]
fn test_decode_to_buf_b64_len_equal_to_b64_len_to_decode() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { 
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) 
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    reader.b64_len = 3;
    let mut buf = [0u8; 3];
    let result = reader.decode_to_buf(3, &mut buf);
}

#[test]
fn test_decode_to_buf_b64_len_greater_than_buf_size() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { 
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) 
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    reader.b64_len = BUF_SIZE;
    let mut buf = [0u8; BUF_SIZE];
    let result = reader.decode_to_buf(BUF_SIZE + 1, &mut buf);
}

#[test]
fn test_decode_to_buf_b64_len_zero() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> { 
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None }) 
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut reader = DecoderReader::new(std::io::empty(), &engine);
    reader.b64_len = 3;
    let mut buf = [0u8; 3];
    let result = reader.decode_to_buf(0, &mut buf);
}

