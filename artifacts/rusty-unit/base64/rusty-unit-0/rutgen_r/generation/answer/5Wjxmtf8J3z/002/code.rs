// Answer 0

#[test]
fn test_decode_to_buf_exact_length() {
    struct TestEngine;

    impl TestEngine {
        fn internal_decode(&self, _: &[u8], buf: &mut [u8], _: usize) -> Result<DecodeMetadata, DecodeSliceError> {
            if buf.len() >= 4 {
                buf[..4].copy_from_slice(&[1, 2, 3, 4]);
                Ok(DecodeMetadata { decoded_len: 4, padding_offset: None })
            } else {
                Err(DecodeSliceError::OutputSliceTooSmall)
            }
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> usize {
            4
        }
    }

    struct Decoder {
        b64_buffer: Vec<u8>,
        b64_offset: usize,
        b64_len: usize,
        engine: TestEngine,
        padding_offset: Option<usize>,
        input_consumed_len: usize,
    }

    let mut buf = [0u8; 4];
    let mut decoder = Decoder {
        b64_buffer: vec![b'A', b'B', b'C', b'D'],
        b64_offset: 0,
        b64_len: 4,
        engine: TestEngine,
        padding_offset: None,
        input_consumed_len: 0,
    };

    let result = decoder.decode_to_buf(4, &mut buf);
    
    assert_eq!(result.unwrap(), 4);
    assert_eq!(&buf, &[1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_decode_to_buf_panic_empty_buffer() {
    struct TestEngine;

    impl TestEngine {
        fn internal_decode(&self, _: &[u8], buf: &mut [u8], _: usize) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None })
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> usize {
            0
        }
    }

    struct Decoder {
        b64_buffer: Vec<u8>,
        b64_offset: usize,
        b64_len: usize,
        engine: TestEngine,
        padding_offset: Option<usize>,
        input_consumed_len: usize,
    }

    let mut decoder = Decoder {
        b64_buffer: vec![b'A', b'B', b'C', b'D'],
        b64_offset: 0,
        b64_len: 4,
        engine: TestEngine,
        padding_offset: None,
        input_consumed_len: 0,
    };

    let mut buf: [u8; 0] = [];
    decoder.decode_to_buf(4, &mut buf); // This should panic
}

#[test]
fn test_decode_to_buf_partial_decoding() {
    struct TestEngine;

    impl TestEngine {
        fn internal_decode(&self, _: &[u8], buf: &mut [u8], _: usize) -> Result<DecodeMetadata, DecodeSliceError> {
            if buf.len() >= 2 {
                buf[..2].copy_from_slice(&[10, 20]);
                Ok(DecodeMetadata { decoded_len: 2, padding_offset: None })
            } else {
                Err(DecodeSliceError::OutputSliceTooSmall)
            }
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> usize {
            2
        }
    }

    struct Decoder {
        b64_buffer: Vec<u8>,
        b64_offset: usize,
        b64_len: usize,
        engine: TestEngine,
        padding_offset: Option<usize>,
        input_consumed_len: usize,
    }

    let mut buf = [0u8; 2];
    let mut decoder = Decoder {
        b64_buffer: vec![b'A', b'B', b'C', b'D'],
        b64_offset: 0,
        b64_len: 4,
        engine: TestEngine,
        padding_offset: None,
        input_consumed_len: 0,
    };

    let result = decoder.decode_to_buf(2, &mut buf);
    
    assert_eq!(result.unwrap(), 2);
    assert_eq!(&buf, &[10, 20]);
}

