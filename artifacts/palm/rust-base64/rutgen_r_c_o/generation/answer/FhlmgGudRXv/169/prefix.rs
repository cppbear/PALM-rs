// Answer 0

#[test]
fn test_read_partial_decoded_data() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_bytes = match input {
                b"MQ" => {
                    output[0] = 1;
                    output[1] = 0;
                    output[2] = 0;
                    2
                }
                _ => return Err(DecodeSliceError::OutputSliceTooSmall),
            };
            Ok(DecodeMetadata { decoded_len: decoded_bytes, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut buf = [0u8; 2];
    let reader = DecoderReader::new(std::io::empty(), &engine);

    let mut decoder = reader;
    decoder.b64_offset = 0;
    decoder.b64_len = 4;
    decoder.decoded_len = 2;
    decoder.decoded_offset = 1;

    let result = decoder.read(&mut buf);
}

#[test]
fn test_read_complete_decoded_data() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_bytes = match input {
                b"MQ==\n" => {
                    output[0] = 1;
                    output[1] = 0;
                    output[2] = 0;
                    3
                }
                _ => return Err(DecodeSliceError::OutputSliceTooSmall),
            };
            Ok(DecodeMetadata { decoded_len: decoded_bytes, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut buf = [0u8; 3];
    let reader = DecoderReader::new(std::io::empty(), &engine);

    let mut decoder = reader;
    decoder.b64_offset = 0;
    decoder.b64_len = 4;
    decoder.decoded_len = 3;
    decoder.decoded_offset = 0;

    let result = decoder.read(&mut buf);
}

#[test]
fn test_read_eof_with_remaining_data() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            let decoded_bytes = match input {
                b"MQ==" => {
                    output[0] = 1;
                    output[1] = 0;
                    output[2] = 0;
                    3
                }
                _ => {
                    return Err(DecodeSliceError::OutputSliceTooSmall);
                }
            };
            Ok(DecodeMetadata { decoded_len: decoded_bytes, padding_offset: None })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = MockEngine;
    let mut buf = [0u8; 2];
    let reader = DecoderReader::new(std::io::empty(), &engine);

    let mut decoder = reader;
    decoder.b64_offset = 0;
    decoder.b64_len = 4;
    decoder.decoded_len = 3;
    decoder.decoded_offset = 0;

    let result = decoder.read(&mut buf);
}

