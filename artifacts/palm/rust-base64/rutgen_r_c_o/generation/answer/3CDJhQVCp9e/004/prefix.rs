// Answer 0

#[test]
fn test_write_final_leftovers_with_valid_inputs() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 3 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let data = input.as_ref();
            output_buf[..data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    let engine = MockEngine;
    let delegate: Vec<u8> = vec![0; 1024];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(delegate),
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write_final_leftovers();
}

#[test]
fn test_write_final_leftovers_with_partial_input() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 2 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let data = input.as_ref();
            output_buf[..data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    let engine = MockEngine;
    let delegate: Vec<u8> = vec![0; 1024];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(delegate),
        extra_input: [4, 5, 6],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write_final_leftovers();
}

#[test]
fn test_write_final_leftovers_with_minimum_input_size() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 1 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let data = input.as_ref();
            output_buf[..data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    let engine = MockEngine;
    let delegate: Vec<u8> = vec![0; 1024];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(delegate),
        extra_input: [7, 0, 0],
        extra_input_occupied_len: 1,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write_final_leftovers();
}

#[test]
fn test_write_final_leftovers_with_maximum_output_capacity() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { BUF_SIZE }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }
        fn config(&self) -> &Self::Config { &() }
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let data = input.as_ref();
            output_buf[..data.len()].copy_from_slice(data);
            Ok(data.len())
        }
    }

    let engine = MockEngine;
    let delegate: Vec<u8> = vec![0; 1024];
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(delegate),
        extra_input: [8, 9, 10],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let _ = encoder_writer.write_final_leftovers();
}

