// Answer 0

fn write_final_leftovers_test() -> Result<()> {
    struct MockEngine;

    impl MockEngine {
        fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize> {
            let len = input.len().min(output.len());
            output[..len].copy_from_slice(&input[..len]);
            Ok(len)
        }
    }

    struct MockWriter {
        called: bool,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { called: false }
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            self.called = true;
            Ok(())
        }
    }

    struct Encoder {
        delegate: Option<MockWriter>,
        engine: MockEngine,
        extra_input: Vec<u8>,
        extra_input_occupied_len: usize,
        output: Vec<u8>,
        output_occupied_len: usize,
    }

    impl Encoder {
        fn new(delegate: MockWriter) -> Self {
            Self {
                delegate: Some(delegate),
                engine: MockEngine,
                extra_input: vec![1, 2, 3, 4, 5],
                extra_input_occupied_len: 5,
                output: vec![0; 10],
                output_occupied_len: 0,
            }
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            self.delegate.as_mut().unwrap().write_all_encoded_output()
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

    let mut writer = MockWriter::new();
    let mut encoder = Encoder::new(writer);
    let result = encoder.write_final_leftovers()?;
    
    assert!(result.is_ok());
    assert!(encoder.output_occupied_len > 0);
    assert!(encoder.extra_input_occupied_len == 0);
    
    Ok(())
}

#[test]
fn test_write_final_leftovers() {
    write_final_leftovers_test().unwrap();
}

