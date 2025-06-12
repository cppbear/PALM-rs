// Answer 0

#[test]
fn test_string_sink_new() {
    let mut output = String::new();
    let sink = StringSink::new(&mut output);
    assert_eq!(output, "");
}

#[test]
fn test_string_sink_write_encoded_bytes() {
    struct TestSink {
        encoded: Vec<u8>,
        error_occurred: bool,
    }

    impl Sink for TestSink {
        type Error = &'static str;

        fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error> {
            if self.error_occurred {
                return Err("Error occurred");
            }
            self.encoded.extend_from_slice(encoded);
            Ok(())
        }
    }

    let mut test_sink = TestSink {
        encoded: Vec::new(),
        error_occurred: false,
    };

    let result = test_sink.write_encoded_bytes(b"test");
    assert!(result.is_ok());
    assert_eq!(test_sink.encoded, b"test");

    test_sink.error_occurred = true;
    let result_with_error = test_sink.write_encoded_bytes(b"test");
    assert_eq!(result_with_error, Err("Error occurred"));
}

