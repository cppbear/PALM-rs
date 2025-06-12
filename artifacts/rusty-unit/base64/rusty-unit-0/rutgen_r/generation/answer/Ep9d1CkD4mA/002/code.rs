// Answer 0

struct MockWriter {
    should_interrupt: bool,
    output_written: bool,
}

impl MockWriter {
    fn new(should_interrupt: bool) -> Self {
        Self {
            should_interrupt,
            output_written: false,
        }
    }

    fn write_to_delegate(&mut self, len: usize) -> Result<(), std::io::Error> {
        if self.should_interrupt {
            Err(std::io::Error::from(std::io::ErrorKind::Interrupted))
        } else {
            self.output_written = true;
            Ok(())
        }
    }
}

struct Encoder {
    output_occupied_len: usize,
    writer: MockWriter,
}

impl Encoder {
    fn new(output_length: usize, writer: MockWriter) -> Self {
        Self {
            output_occupied_len: output_length,
            writer,
        }
    }

    fn write_all_encoded_output(&mut self) -> Result<()> {
        while self.output_occupied_len > 0 {
            let remaining_len = self.output_occupied_len;
            match self.writer.write_to_delegate(remaining_len) {
                Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
                Ok(()) => {},
            };
        }

        debug_assert_eq!(0, self.output_occupied_len);
        Ok(())
    }
}

#[test]
fn test_write_all_encoded_output_interrupted() {
    let mut encoder = Encoder::new(10, MockWriter::new(true));
    let result = encoder.write_all_encoded_output();
    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 10);
}

#[test]
fn test_write_all_encoded_output_without_interrupt() {
    let mut encoder = Encoder::new(10, MockWriter::new(false));
    let result = encoder.write_all_encoded_output();
    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 0);
}

#[test]
fn test_write_all_encoded_output_errors() {
    let mut encoder = Encoder::new(10, MockWriter::new(false));
    encoder.writer.should_interrupt = false;
    let mut result = encoder.write_all_encoded_output();
    assert!(result.is_ok());
    
    encoder.output_occupied_len = 5; // Simulate an error after some writes
    encoder.writer.should_interrupt = false;
    result = encoder.write_all_encoded_output();
    assert!(result.is_err());
}

#[test]
fn test_write_all_encoded_output_no_output() {
    let mut encoder = Encoder::new(0, MockWriter::new(false));
    let result = encoder.write_all_encoded_output();
    assert!(result.is_ok());
    assert_eq!(encoder.output_occupied_len, 0);
}

