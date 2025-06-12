// Answer 0

#[test]
fn test_encoder_writer_flush() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let encoded_len = std::cmp::min(input.len() * 4 / 3, output.len());
            output[..encoded_len].copy_from_slice(&input[..encoded_len]);
            encoded_len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len * 3 / 4
        }

        fn internal_decode(&self, input: &[u8], output: &mut [u8], _: Self::DecodeEstimate) -> Result<(), io::Error> {
            output[..input.len()].copy_from_slice(input);
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter {
        data: Vec<u8>,
        flushed: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            self.flushed = true;
            Ok(())
        }
    }

    let engine = MockEngine;
    let writer = MockWriter {
        data: Vec::new(),
        flushed: false,
    };

    let mut encoder_writer = EncoderWriter::new(writer, &engine);
    let result = encoder_writer.flush();
    
    assert!(result.is_ok());
    assert!(encoder_writer.delegate.is_some());
}

