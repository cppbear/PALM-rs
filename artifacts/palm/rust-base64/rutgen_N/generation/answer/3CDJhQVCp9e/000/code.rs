// Answer 0

#[test]
fn test_write_final_leftovers_with_none_delegate() {
    struct NoOpWriter;

    impl std::io::Write for NoOpWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Ok(0)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<NoOpWriter>,
        extra_input_occupied_len: usize,
        extra_input: Vec<u8>,
        output: Vec<u8>,
        output_occupied_len: usize,
        engine: EncoderEngine,
    }

    struct EncoderEngine;

    impl EncoderEngine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize, std::io::Error> {
        // Simulate encoding by copying to output
            for (i, byte) in input.iter().enumerate() {
                output[i] = byte + 1; // Simple encoding: increment each byte
            }
            Ok(input.len())
        }
    }

    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<(), std::io::Error> {
            if let Some(ref mut writer) = self.delegate {
                writer.write(&self.output[..self.output_occupied_len])?;
            }
            Ok(())
        }

        fn write_final_leftovers(&mut self) -> Result<(), std::io::Error> {
            if self.delegate.is_none() {
                return Ok(());
            }

            self.write_all_encoded_output()?;

            if self.extra_input_occupied_len > 0 {
                let encoded_len = self
                    .engine
                    .encode_slice(
                        &self.extra_input[..self.extra_input_occupied_len],
                        &mut self.output[..],
                    )?;

                self.output_occupied_len = encoded_len;

                self.write_all_encoded_output()?;

                self.extra_input_occupied_len = 0;
            }

            Ok(())
        }
    }

    let encoder = Encoder {
        delegate: None,
        extra_input_occupied_len: 0,
        extra_input: vec![1, 2, 3],
        output: vec![0; 10],
        output_occupied_len: 0,
        engine: EncoderEngine,
    };

    let mut encoder = encoder;

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
}

#[test]
fn test_write_final_leftovers_with_some_delegate_and_no_extra_input() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { written: Vec::new() };

    let mut encoder = Encoder {
        delegate: Some(writer),
        extra_input_occupied_len: 0,
        extra_input: Vec::new(),
        output: vec![0; 10],
        output_occupied_len: 0,
        engine: EncoderEngine,
    };

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
    assert!(encoder.delegate.is_some()); // Make sure delegate is still intact
}

#[test]
fn test_write_final_leftovers_with_some_delegate_and_extra_input() {
    struct MockWriter {
        written: Vec<u8>,
    }

    impl std::io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { written: Vec::new() };

    let mut encoder = Encoder {
        delegate: Some(writer),
        extra_input_occupied_len: 3,
        extra_input: vec![1, 2, 3],
        output: vec![0; 10],
        output_occupied_len: 0,
        engine: EncoderEngine,
    };

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
    assert!(encoder.delegate.is_some());
    assert_eq!(encoder.extra_input_occupied_len, 0); // Ensure extra input is reset
}

