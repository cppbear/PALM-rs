// Answer 0

#[test]
fn test_write_all_encoded_output_no_buffer() {
    struct TestWriter {
        output_occupied_len: usize,
    }

    impl TestWriter {
        fn write_all_encoded_output(&mut self) -> Result<()> {
            while self.output_occupied_len > 0 {
                let remaining_len = self.output_occupied_len;
                match self.write_to_delegate(remaining_len) {
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                    Ok(()) => {}
                };
            }

            debug_assert_eq!(0, self.output_occupied_len);
            Ok(())
        }

        fn write_to_delegate(&mut self, _: usize) -> Result<()> {
            // Simulate successful write
            self.output_occupied_len = 0; // Ensuring we reach the termination condition
            Ok(())
        }
    }

    let mut writer = TestWriter { output_occupied_len: 0 };
    let result = writer.write_all_encoded_output();
    assert!(result.is_ok());
}

#[test]
fn test_write_all_encoded_output_with_interrupted() {
    struct TestWriter {
        output_occupied_len: usize,
        interrupted: bool,
    }

    impl TestWriter {
        fn write_all_encoded_output(&mut self) -> Result<()> {
            while self.output_occupied_len > 0 {
                let remaining_len = self.output_occupied_len;
                match self.write_to_delegate(remaining_len) {
                    Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                    Ok(()) => {}
                };
            }

            debug_assert_eq!(0, self.output_occupied_len);
            Ok(())
        }

        fn write_to_delegate(&mut self, _: usize) -> Result<()> {
            if self.interrupted {
                return Err(std::io::Error::new(std::io::ErrorKind::Interrupted, "Interrupted"));
            }
            self.output_occupied_len = 0; // Writing simulates completing the write
            Ok(())
        }
    }

    let mut writer = TestWriter { output_occupied_len: 1, interrupted: true };
    let result = writer.write_all_encoded_output();
    assert!(result.is_ok());
    assert_eq!(writer.output_occupied_len, 0);
}

