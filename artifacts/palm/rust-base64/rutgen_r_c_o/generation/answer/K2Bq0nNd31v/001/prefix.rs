// Answer 0

#[test]
fn test_encoder_string_writer_with_valid_consumer_and_engine() {
    struct TestEngine;
    struct TestStrConsumer {
        consumed: String,
    }

    impl Send for TestEngine {}
    impl Sync for TestEngine {}
    
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].clone_from_slice(input);
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
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let engine = TestEngine;
    let mut consumer = TestStrConsumer { consumed: String::new() };
    let _writer = EncoderStringWriter::from_consumer(consumer, &engine);
}

#[test]
fn test_encoder_string_writer_with_different_consumer_and_engine() {
    struct AnotherTestEngine;
    struct AnotherTestStrConsumer {
        consumed: String,
    }

    impl Send for AnotherTestEngine {}
    impl Sync for AnotherTestEngine {}

    impl Engine for AnotherTestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].clone_from_slice(input);
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
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    impl StrConsumer for AnotherTestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.consumed.push_str(buf);
        }
    }

    let engine = AnotherTestEngine;
    let mut consumer = AnotherTestStrConsumer { consumed: String::new() };
    let _writer = EncoderStringWriter::from_consumer(consumer, &engine);
}

#[test]
#[should_panic]
fn test_encoder_string_writer_with_panic_on_consume() {
    struct PanickingStrConsumer;

    impl StrConsumer for PanickingStrConsumer {
        fn consume(&mut self, _: &str) {
            panic!("This is a test panic!");
        }
    }

    struct PanickingEngine;

    impl Send for PanickingEngine {}
    impl Sync for PanickingEngine {}

    impl Engine for PanickingEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = PanickingEngine;
    let mut consumer = PanickingStrConsumer;
    let _writer = EncoderStringWriter::from_consumer(consumer, &engine);
}

