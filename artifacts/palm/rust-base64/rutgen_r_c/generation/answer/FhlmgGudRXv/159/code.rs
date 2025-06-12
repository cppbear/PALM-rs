// Answer 0

#[test]
fn test_read_empty_buffer() {
    let mock_engine = MockEngine::new();
    let input: &[u8] = b"";
    let mut reader = DecoderReader::new(&mut input, &mock_engine);
    let mut buf = Vec::new();
    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_read_no_data() {
    let mock_engine = MockEngine::new();
    let input: &[u8] = b"";
    let mut reader = DecoderReader::new(&mut input, &mock_engine);
    
    let mut buf = [0u8; 3];
    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(0));
}

#[test]
fn test_read_at_eof_with_full_buffer() {
    let mock_engine = MockEngine::new();
    let input: &[u8] = b"QUJD"; // "ABC" in base64
    let mut reader = DecoderReader::new(&mut input, &mock_engine);
    
    let mut buf = [0u8; 3];
    reader.b64_offset = BUF_SIZE - BASE64_CHUNK_SIZE; 
    reader.b64_len = BUF_SIZE; 
    reader.decoded_len = 0; 
    
    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(3));
    assert_eq!(&buf[..3], b"ABC");
}

#[test]
fn test_read_at_eof_with_partial_buffer() {
    let mock_engine = MockEngine::new();
    let input: &[u8] = b"QUE"; // "AB" in base64 with padding
    let mut reader = DecoderReader::new(&mut input, &mock_engine);
    
    let mut buf = [0u8; 3];
    reader.b64_offset = BUF_SIZE - BASE64_CHUNK_SIZE;
    reader.b64_len = BUF_SIZE; 
    reader.decoded_len = 0; 
    
    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(2));
    assert_eq!(&buf[..2], b"AB");
}

#[test]
fn test_read_with_full_decoded_content() {
    let mock_engine = MockEngine::new();
    let input: &[u8] = b"QUJDQUJD"; // "ABCABC" in base64
    let mut reader = DecoderReader::new(&mut input, &mock_engine);
    
    let mut buf = [0u8; 6];
    reader.b64_offset = BUF_SIZE - BASE64_CHUNK_SIZE;
    reader.b64_len = BUF_SIZE; 
    reader.decoded_len = 3; // Simulate that we already decoded 3 bytes
    
    let result = reader.read(&mut buf);
    assert_eq!(result, Ok(3));
    assert_eq!(&buf[..3], b"ABC");
}

struct MockEngine;

impl MockEngine {
    fn new() -> Self {
        MockEngine {}
    }
}

impl Engine for MockEngine {
    type Config = ();
    type DecodeEstimate = usize;

    fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
        unimplemented!()
    }

    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        input_len / 4 * 3
    }

    fn internal_decode(&self, input: &[u8], output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
        if input == b"QUJD" {
            output.copy_from_slice(b"ABC");
            Ok(DecodeMetadata { decoded_len: 3, padding_offset: None })
        } else if input == b"QUE" {
            output.copy_from_slice(b"AB");
            Ok(DecodeMetadata { decoded_len: 2, padding_offset: None })
        } else {
            Err(DecodeSliceError::DecodeError(DecodeError::InvalidByte(0, 0)))
        }
    }

    fn config(&self) -> &Self::Config {
        &()
    }
}

struct DecodeMetadata {
    decoded_len: usize,
    padding_offset: Option<usize>,
}

#[derive(Debug)]
enum DecodeError {
    InvalidByte(usize, u8),
    InvalidLength(usize),
    InvalidLastSymbol(usize, u8),
    InvalidPadding,
}

#[derive(Debug)]
enum DecodeSliceError {
    DecodeError(DecodeError),
    OutputSliceTooSmall,
}

