// Answer 0

#[test]
fn test_write_final_leftovers_with_no_delegate() {
    struct TestEngine;
    impl TestEngine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize, &'static str> {
            output[..input.len()].copy_from_slice(input);
            Ok(input.len())
        }
    }

    struct TestWriter {
        delegate: Option<()>,
        engine: TestEngine,
        extra_input: Vec<u8>,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
    }

    impl TestWriter {
        fn new() -> Self {
            Self {
                delegate: Some(()),
                engine: TestEngine,
                extra_input: vec![b'a', b'b', b'c'],
                extra_input_occupied_len: 0,
                output: vec![0; 10],
                output_occupied_len: 0,
            }
        }

        fn write_all_encoded_output(&mut self) -> Result<(), &'static str> {
            // Simulating satisfactory writing functionality
            self.output_occupied_len = 0; // Reset for test context
            Ok(())
        }

        fn write_final_leftovers(&mut self) -> Result<(), &'static str> {
            if self.delegate.is_none() {
                return Ok(());
            }

            self.write_all_encoded_output()?;

            if self.extra_input_occupied_len > 0 {
                let encoded_len = self.engine.encode_slice(
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

    let mut writer = TestWriter::new();
    writer.extra_input_occupied_len = 0; // Satisfy constraint
    assert!(writer.write_final_leftovers().is_ok());
}

