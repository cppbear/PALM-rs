// Answer 0

#[test]
fn test_write_final_leftovers_success_with_interrupted_error() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
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
        ) -> Result<(), DecodeSliceError> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            base64::encode(input)
        }

        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            let input_ref = input.as_ref();
            if output_buf.len() < (input_ref.len() + 1) / 3 * 4 {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            let len = self.internal_encode(input_ref, output_buf);
            Ok(len)
        }
    }

    struct MockWriter {
        should_error: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_error {
                Err(io::Error::new(ErrorKind::Interrupted, "write interrupted"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mock_writer = MockWriter { should_error: true };
    let mut encoder = EncoderWriter::new(mock_writer, &engine);

    encoder.extra_input_occupied_len = 2;
    encoder.output_occupied_len = 10;

    encoder.write_final_leftovers().unwrap();
}

#[test]
fn test_write_final_leftovers_with_error() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
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
        ) -> Result<(), DecodeSliceError> {
            Ok(())
        }

        fn config(&self) -> &Self::Config {
            &()
        }

        fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
            base64::encode(input)
        }

        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            let input_ref = input.as_ref();
            if output_buf.len() < (input_ref.len() + 1) / 3 * 4 {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            let len = self.internal_encode(input_ref, output_buf);
            Ok(len)
        }
    }

    struct MockWriter {
        should_error: bool,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_error {
                Err(io::Error::new(ErrorKind::Other, "write error"))
            } else {
                Ok(buf.len())
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let mock_writer = MockWriter { should_error: true };
    let mut encoder = EncoderWriter::new(mock_writer, &engine);

    encoder.extra_input_occupied_len = 2;
    encoder.output_occupied_len = 10;

    encoder.write_final_leftovers().unwrap_err();
}

