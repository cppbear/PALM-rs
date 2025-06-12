// Answer 0

#[test]
fn test_into_inner_with_valid_delegate() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<(), ()> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let writer: Vec<u8> = Vec::new();
    let mut encoder_writer = EncoderWriter::new(writer, &mock_engine);
    let inner_writer = encoder_writer.into_inner();
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_into_inner_after_finish() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<(), ()> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let writer: Vec<u8> = Vec::new();
    let mut encoder_writer = EncoderWriter::new(writer, &mock_engine);
    let _ = encoder_writer.finish();
    let _ = encoder_writer.into_inner();
}

#[test]
fn test_into_inner_with_max_buffer_usage() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<(), ()> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let mut writer: Vec<u8> = Vec::with_capacity(BUF_SIZE);
    let mut encoder_writer = EncoderWriter::new(writer, &mock_engine);

    encoder_writer.extra_input_occupied_len = MIN_ENCODE_CHUNK_SIZE;
    encoder_writer.output_occupied_len = BUF_SIZE;

    let inner_writer = encoder_writer.into_inner();
}

#[test]
fn test_into_inner_with_none_delegate() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _: &[u8], _: &mut [u8], _: Self::DecodeEstimate) -> Result<(), ()> { Ok(()) }
        fn config(&self) -> &Self::Config { &() }
    }

    let mock_engine = MockEngine;
    let mut encoder_writer: EncoderWriter<_, Vec<u8>> = EncoderWriter::new(vec![], &mock_engine);
    let _ = encoder_writer.finish();

    // Now `delegate` should be None, to test the edge case.
    encoder_writer.delegate = None;

    let result = std::panic::catch_unwind(|| {
        let _ = encoder_writer.into_inner();
    });

    assert!(result.is_err());  // Check that it panics
}

