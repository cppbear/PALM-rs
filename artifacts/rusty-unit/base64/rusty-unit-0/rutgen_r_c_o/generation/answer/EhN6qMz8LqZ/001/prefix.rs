// Answer 0

#[test]
fn test_into_inner_with_empty_consumer() {
    struct MockConsumer {
        buffer: String,
    }

    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.buffer.push_str(buf);
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let consumer = MockConsumer { buffer: String::new() };
    let engine = MockEngine;
    let encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    let final_consumer = encoder_string_writer.into_inner();
}

#[test]
fn test_into_inner_with_partial_input() {
    struct MockConsumer {
        buffer: String,
    }

    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.buffer.push_str(buf);
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let consumer = MockConsumer { buffer: String::new() };
    let engine = MockEngine;
    let mut encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    encoder_string_writer.encoder.extra_input_occupied_len = 1; // Simulate a partial input
    let final_consumer = encoder_string_writer.into_inner();
}

#[test]
fn test_into_inner_with_full_output() {
    struct MockConsumer {
        buffer: String,
    }

    impl StrConsumer for MockConsumer {
        fn consume(&mut self, buf: &str) {
            self.buffer.push_str(buf);
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output.copy_from_slice(input);
            input.len()
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
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let consumer = MockConsumer { buffer: String::new() };
    let engine = MockEngine;
    let encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    encoder_string_writer.encoder.output_occupied_len = BUF_SIZE; // Fill output buffer
    let final_consumer = encoder_string_writer.into_inner();
}

