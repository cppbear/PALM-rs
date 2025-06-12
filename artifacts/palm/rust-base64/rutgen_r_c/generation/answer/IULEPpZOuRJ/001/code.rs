// Answer 0

#[test]
fn test_into_inner_no_finish_called() {
    struct DummyEngine;
    struct DummyWriter;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata)
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let writer = DummyWriter;
    let engine = DummyEngine;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    let inner_writer = encoder_writer.into_inner();
    // Check that the inner writer was returned correctly
    assert_eq!(inner_writer, writer);
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_into_inner_after_finish_called() {
    struct DummyEngine;
    struct DummyWriter;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;
        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate { 0 }
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata)
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let writer = DummyWriter;
    let engine = DummyEngine;
    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    // Simulate finishing the encoder writer
    encoder_writer.finish();
    // Attempt to get the inner writer after it has been finished
    encoder_writer.into_inner();
}

