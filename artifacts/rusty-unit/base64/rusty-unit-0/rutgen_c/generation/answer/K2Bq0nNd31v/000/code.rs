// Answer 0

#[test]
fn test_encoder_string_writer_from_consumer() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0 // dummy implementation
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0 // dummy implementation
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError) // dummy implementation
        }

        fn config(&self) -> &Self::Config {
            &() // dummy implementation
        }
    }

    struct TestStrConsumer {
        buffer: String,
    }

    impl StrConsumer for TestStrConsumer {
        fn consume(&mut self, buf: &str) {
            self.buffer.push_str(buf);
        }
    }

    let engine = TestEngine;
    let consumer = TestStrConsumer { buffer: String::new() };
    let encoder_string_writer = EncoderStringWriter::from_consumer(consumer, &engine);
    
    assert_eq!(encoder_string_writer.encoder.engine, &engine);
}

