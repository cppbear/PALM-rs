// Answer 0

#[test]
fn test_decoder_reader_read_with_valid_data() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len * 3 / 4
        }
        
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_len = base64::decode(input).map_err(|_| DecodeSliceError::OutputSliceTooSmall)?;
            output[..decoded_len.len()].copy_from_slice(&decoded_len);
            Ok(DecodeMetadata { decoded_len: decoded_len.len(), padding_offset: None })
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let input_data = "SGVsbG8sIFdvcmxkIQ=="; // "Hello, World!" in Base64
    let mock_reader = input_data.as_bytes();
    let engine = MockEngine;
    let mut decoder = DecoderReader::new(mock_reader, &engine);
    
    let mut buffer = [0; 3];
    let result = decoder.read(&mut buffer).unwrap();
    
    assert_eq!(result, 3);
    assert_eq!(&buffer, b"Hel");
}

#[test]
fn test_decoder_reader_read_with_partial_decode() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len * 3 / 4
        }
        
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_len = base64::decode(input).map_err(|_| DecodeSliceError::OutputSliceTooSmall)?;
            output[..decoded_len.len()].copy_from_slice(&decoded_len);
            Ok(DecodeMetadata { decoded_len: decoded_len.len(), padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let input_data = "SGVs"; // "Hel" part of "Hello, World!" in Base64
    let mock_reader = input_data.as_bytes();
    let engine = MockEngine;
    let mut decoder = DecoderReader::new(mock_reader, &engine);
    
    let mut buffer = [0; 3];
    let result = decoder.read(&mut buffer).unwrap();
    
    assert_eq!(result, 2); // Only "He" should be decoded (SGV -> He)
    assert_eq!(&buffer[..2], b"He");
}

#[test]
fn test_decoder_reader_read_empty_buf() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len * 3 / 4
        }
        
        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let input_data = "SGVsbG8sIFdvcmxkIQ=="; // "Hello, World!" in Base64
    let mock_reader = input_data.as_bytes();
    let engine = MockEngine;
    let mut decoder = DecoderReader::new(mock_reader, &engine);
    
    let mut buffer = [];
    let result = decoder.read(&mut buffer).unwrap();

    assert_eq!(result, 0);
}

