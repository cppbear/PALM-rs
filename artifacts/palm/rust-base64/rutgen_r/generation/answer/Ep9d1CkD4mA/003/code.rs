// Answer 0

fn write_all_encoded_output_err() -> Result<()> {
    use std::io::{Error, ErrorKind, Result};

    struct TestWriter {
        output_occupied_len: usize,
        next_return: Result<()>,
    }

    impl TestWriter {
        fn new(output_occupied_len: usize, next_return: Result<()>) -> Self {
            TestWriter {
                output_occupied_len,
                next_return,
            }
        }

        fn write_to_delegate(&mut self, _remaining_len: usize) -> Result<()> {
            // Always return the next_result set in the struct
            self.next_return.clone()
        }

        fn write_all_encoded_output(&mut self) -> Result<()> {
            while self.output_occupied_len > 0 {
                let remaining_len = self.output_occupied_len;
                match self.write_to_delegate(remaining_len) {
                    Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                    Err(e) => return Err(e),
                    Ok(()) => {}
                };
            }
            debug_assert_eq!(0, self.output_occupied_len);
            Ok(())
        }
    }

    #[test]
    fn test_write_all_encoded_output_error() {
        let mut writer = TestWriter::new(10, Err(Error::new(ErrorKind::Other, "Test error")));
        let result = writer.write_all_encoded_output();
        assert!(result.is_err());
    }

    #[test]
    fn test_write_all_encoded_output_interrupted() {
        let mut writer = TestWriter::new(10, Err(Error::new(ErrorKind::Interrupted, "interrupted")));
        let result = writer.write_all_encoded_output();
        assert!(result.is_ok()); // should continue and not return error
    }

    #[test]
    fn test_write_all_encoded_output_multiple_errors() {
        let mut writer = TestWriter::new(10, Err(Error::new(ErrorKind::Other, "Test error 1")));
        
        // First call should return error
        let result_first = writer.write_all_encoded_output();
        assert!(result_first.is_err());

        // Change to a different error to ensure state is kept
        writer.next_return = Err(Error::new(ErrorKind::Other, "Test error 2"));
        let result_second = writer.write_all_encoded_output();
        assert!(result_second.is_err());
    }
}

