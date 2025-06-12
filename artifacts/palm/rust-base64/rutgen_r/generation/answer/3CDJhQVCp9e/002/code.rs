// Answer 0


struct MockDelegateWriter {
    should_fail: bool,
}

impl MockDelegateWriter {
    fn new(should_fail: bool) -> Self {
        Self { should_fail }
    }
}

impl Write for MockDelegateWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.should_fail {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write failed"))
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

struct Encoder {
    delegate: Option<MockDelegateWriter>,
    output: Vec<u8>,
    output_occupied_len: usize,
    extra_input: Vec<u8>,
    extra_input_occupied_len: usize,
    engine: MockEngine,
}

impl Encoder {
    fn new(delegate: MockDelegateWriter) -> Self {
        Self {
            delegate: Some(delegate),
            output: vec![0; 64],
            output_occupied_len: 0,
            extra_input: vec![0; 32],
            extra_input_occupied_len: 0,
            engine: MockEngine {},
        }
    }

    fn write_all_encoded_output(&mut self) -> Result<()> {
        // Simulate write behavior
        if let Some(writer) = &mut self.delegate {
            writer.write(&self.output[..self.output_occupied_len])?;
        }
        Ok(())
    }
}

struct MockEngine {}

impl MockEngine {
    fn encode_slice(&self, input: &[u8], output: &mut [u8]) -> Result<usize> {
        // Simulate encoding behavior
        let encoded_len = input.len(); // Just echo the input size for simplicity
        output[..encoded_len].copy_from_slice(input);
        Ok(encoded_len)
    }
}

#[test]
fn test_write_final_leftovers_success() {
    let mock_writer = MockDelegateWriter::new(false);
    let mut encoder = Encoder::new(mock_writer);
    encoder.extra_input_occupied_len = 8; // Set to more than 0 to trigger extra write
    encoder.extra_input[..8].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "write failed")]
fn test_write_final_leftovers_write_failure() {
    let mock_writer = MockDelegateWriter::new(true);
    let mut encoder = Encoder::new(mock_writer);
    encoder.extra_input_occupied_len = 8; // Set to more than 0 to trigger extra write
    encoder.extra_input[..8].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);

    encoder.write_final_leftovers().unwrap(); // This should panic
}

#[test]
fn test_write_final_leftovers_no_extra_input() {
    let mock_writer = MockDelegateWriter::new(false);
    let mut encoder = Encoder::new(mock_writer);
    encoder.extra_input_occupied_len = 0; // No extra input to write

    let result = encoder.write_final_leftovers();
    assert!(result.is_ok()); // Should succeed without any extra input
}


