// Answer 0

#[test]
fn test_read_non_empty_buffer() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            // Simulate valid Base64 decoding
            let decoded_len = input.len() / 4 * 3;
            output[..decoded_len].copy_from_slice(&[0; 3][..decoded_len]);
            Ok(DecodeMetadata { decoded_len })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    use std::io::Cursor;

    let input_data = b"SGVsbG8gd29ybGQ="; // "Hello world" in Base64
    let cursor = Cursor::new(input_data);
    let engine = TestEngine;

    let mut decoder_reader = DecoderReader::new(cursor, &engine);
    let mut output_buf = [0u8; 3];

    let result = decoder_reader.read(&mut output_buf).unwrap();

    assert_eq!(result, 3);
    assert_eq!(&output_buf[..], b"Hel");
}

#[test]
fn test_read_eof_condition() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_len = input.len() / 4 * 3;
            output[..decoded_len].copy_from_slice(&[0; 3][..decoded_len]);
            Ok(DecodeMetadata { decoded_len })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    use std::io::Cursor;

    let input_data = b"SGVsbG8gd29ybGQ="; // "Hello world" in Base64
    let cursor = Cursor::new(input_data);
    let engine = TestEngine;

    let mut decoder_reader = DecoderReader::new(cursor, &engine);
    let mut output_buf = [0u8; 3];

    // Fill the first read
    decoder_reader.read(&mut output_buf).unwrap();

    // Reading again should return 0 bytes read (EOF)
    let result = decoder_reader.read(&mut output_buf).unwrap();
    assert_eq!(result, 0);
}

#[test]
#[should_panic] 
fn test_read_empty_buffer_should_panic() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_len = input.len() / 4 * 3;
            output[..decoded_len].copy_from_slice(&[0; 3][..decoded_len]);
            Ok(DecodeMetadata { decoded_len })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    use std::io::Cursor;

    let input_data = b"SGVsbG8gd29ybGQ="; // "Hello world" in Base64
    let cursor = Cursor::new(input_data);
    let engine = TestEngine;

    let mut decoder_reader = DecoderReader::new(cursor, &engine);
    let mut output_buf = []; // Empty buffer should trigger panic

    let _result = decoder_reader.read(&mut output_buf).unwrap();
}

