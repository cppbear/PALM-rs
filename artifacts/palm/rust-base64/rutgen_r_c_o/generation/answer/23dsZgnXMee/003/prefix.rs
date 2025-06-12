// Answer 0

#[test]
fn test_encode_partial_chunk_without_padding_and_sink_error() {
    struct TestEngine;

    impl Config for TestEngine {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    struct SinkError;

    trait Sink {
        type Error;

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error>;
    }

    struct TestSink {
        should_error: bool,
    }

    impl Sink for TestSink {
        type Error = SinkError;

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            if self.should_error {
                Err(SinkError)
            } else {
                Ok(())
            }
        }
    }

    let engine = TestEngine;
    let encoder = ChunkedEncoder::new(&engine);

    let bytes = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut sink = TestSink { should_error: true };

    encoder.encode(&bytes, &mut sink).unwrap_err();
}

#[test]
fn test_encode_partial_chunk_without_padding_and_sink_error_large_input() {
    struct TestEngine;

    impl Config for TestEngine {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    struct SinkError;

    trait Sink {
        type Error;

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error>;
    }

    struct TestSink {
        should_error: bool,
    }

    impl Sink for TestSink {
        type Error = SinkError;

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            if self.should_error {
                Err(SinkError)
            } else {
                Ok(())
            }
        }
    }

    let engine = TestEngine;
    let encoder = ChunkedEncoder::new(&engine);

    let bytes = vec![0; 1000]; // Large input
    let mut sink = TestSink { should_error: true };

    encoder.encode(&bytes, &mut sink).unwrap_err();
}

