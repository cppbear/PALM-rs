// Answer 0

#[test]
fn test_read_empty_buffer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let reader = std::io::Cursor::new(Vec::new());
    let mut decoder = DecoderReader::new(reader, &engine);
    let mut buf = vec![0u8; 0]; // empty buffer
    let result = decoder.read(&mut buf);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_read_non_empty_buffer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 3 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            _output.copy_from_slice(b"abc");
            Ok(DecodeMetadata { decoded_len: 3 })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"YWJj"; // Base64 for "abc"
    let reader = std::io::Cursor::new(input_data.to_vec());
    let mut decoder = DecoderReader::new(reader, &engine);
    let mut buf = vec![0u8; 3]; // buffer size for three decoded bytes
    let result = decoder.read(&mut buf);
    assert_eq!(result.unwrap(), 3);
    assert_eq!(&buf, b"abc");
}

#[test]
#[should_panic]
fn test_read_invalid_base64() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 3 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(0, 0)))
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"INVALID_BASE64"; 
    let reader = std::io::Cursor::new(input_data.to_vec());
    let mut decoder = DecoderReader::new(reader, &engine);
    let mut buf = vec![0u8; 3];
    let _ = decoder.read(&mut buf);
}

#[test]
fn test_read_small_buffer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 1 }
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            _output[0] = b'a';
            Ok(DecodeMetadata { decoded_len: 1 })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = TestEngine;
    let input_data = b"YQ=="; // Base64 for "a"
    let reader = std::io::Cursor::new(input_data.to_vec());
    let mut decoder = DecoderReader::new(reader, &engine);
    let mut buf = vec![0u8; 1]; // Small buffer
    let result = decoder.read(&mut buf);
    assert_eq!(result.unwrap(), 1);
    assert_eq!(&buf, b"a");
}

