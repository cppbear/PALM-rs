// Answer 0

#[test]
fn test_flush_decoded_buf_non_empty_buffer() {
    struct MockEngine;

    impl Engine for MockEngine {
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
            unimplemented!()
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut decoder = DecoderReader::new(std::io::empty(), &engine);

    decoder.decoded_chunk_buffer[..3].copy_from_slice(&[1, 2, 3]);
    decoder.decoded_len = 3;
    decoder.decoded_offset = 0;

    let mut buf = [0; 3];
    let result = decoder.flush_decoded_buf(&mut buf).unwrap();

    assert_eq!(result, 3);
    assert_eq!(&buf[..result], &[1, 2, 3]);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.decoded_offset, 3);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_empty_input_buffer() {
    struct MockEngine;

    impl Engine for MockEngine {
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
            unimplemented!()
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut decoder = DecoderReader::new(std::io::empty(), &engine);

    decoder.decoded_len = 1;
    decoder.decoded_chunk_buffer[0] = 1;

    let mut buf = [];
    decoder.flush_decoded_buf(&mut buf).unwrap();
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_no_decoded_data() {
    struct MockEngine;

    impl Engine for MockEngine {
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
            unimplemented!()
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let engine = MockEngine;
    let mut decoder = DecoderReader::new(std::io::empty(), &engine);

    decoder.decoded_len = 0;

    let mut buf = [0; 3];
    decoder.flush_decoded_buf(&mut buf).unwrap();
}

