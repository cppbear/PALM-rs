// Answer 0

#[test]
#[should_panic(expected = "Cannot write more after calling finish()")]
fn test_write_when_delegate_is_none() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata::default())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    
    struct DummyWriter;
    impl io::Write for DummyWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: None,
        extra_input: [0; MIN_ENCODE_CHUNK_SIZE],
        extra_input_occupied_len: 0,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    // This will panic because delegate is None
    let _ = encoder_writer.write(&[1, 2, 3]);
}

