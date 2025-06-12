// Answer 0

fn test_write_final_leftovers_valid() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter(Vec<u8>);

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let writer = MockWriter(Vec::new());
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3], // Non-zero length, satisfying the condition
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write_final_leftovers();
    assert!(result.is_ok());
}

fn test_write_final_leftovers_empty_delegate() {
    struct MockEngine;

    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            len
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            todo!()
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct MockWriter;

    impl io::Write for MockWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let engine = MockEngine;
    let writer = MockWriter;
    let mut encoder_writer = EncoderWriter {
        engine: &engine,
        delegate: Some(writer),
        extra_input: [1, 2, 3],
        extra_input_occupied_len: 3,
        output: [0; BUF_SIZE],
        output_occupied_len: 0,
        panicked: false,
    };

    let result = encoder_writer.write_final_leftovers();
    assert!(result.is_err());
}

