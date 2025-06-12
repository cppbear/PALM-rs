// Answer 0

#[test]
fn test_flush_with_empty_writer() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = DummyEngine;
    let writer = Vec::new();
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = 0;
    encoder.extra_input_occupied_len = 1;

    let _ = encoder.flush();
}

#[test]
fn test_flush_with_full_output() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = DummyEngine;
    let writer = Vec::new();
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = BUF_SIZE; // full output
    encoder.extra_input_occupied_len = 2;

    let _ = encoder.flush();
}

#[test]
fn test_flush_with_partial_output() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = DummyEngine;
    let writer = Vec::new();
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = BUF_SIZE / 2; // partial output
    encoder.extra_input_occupied_len = 3;

    let _ = encoder.flush();
}

#[test]
#[should_panic]
fn test_flush_with_writer_panic() {
    struct PanicEngine;

    impl Engine for PanicEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct PanicWriter;

    impl io::Write for PanicWriter {
        fn write(&mut self, _: &[u8]) -> Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "panic"))
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = PanicEngine;
    let writer = PanicWriter;
    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.output_occupied_len = BUF_SIZE / 2;
    encoder.extra_input_occupied_len = 2;

    let _ = encoder.flush();
}

