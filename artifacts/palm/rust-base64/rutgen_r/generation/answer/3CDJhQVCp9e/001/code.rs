// Answer 0

#[test]
fn test_write_final_leftovers_no_delegate() {
    struct TestWriter;

    impl Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<TestWriter>,
        extra_input: Vec<u8>,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
        engine: DummyEngine,
    }

    struct DummyEngine;

    impl DummyEngine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize> {
            let len = input.len();
            if len <= output.len() {
                output[..len].copy_from_slice(input);
                Ok(len)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "buffer too small"))
            }
        }
    }
    
    impl Encoder {
        fn write_all_encoded_output(&mut self) -> Result<()> {
            // Simulation of writing encoded output
            Ok(())
        }

        fn write_final_leftovers(&mut self) -> Result<()> {
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
        extra_input: vec![1, 2, 3],
        extra_input_occupied_len: 3,
        output: vec![0; 10],
        output_occupied_len: 0,
        engine: DummyEngine,
    };

    let result = encoder.write_final_leftovers();
    assert_eq!(result, Ok(()));
}

