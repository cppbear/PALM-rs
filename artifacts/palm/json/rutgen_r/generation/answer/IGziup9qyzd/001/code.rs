// Answer 0


struct TestWriter {
    output: Vec<u8>,
}

impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_pretty_with_empty_writer() {
    let mut writer = TestWriter { output: Vec::new() };
    let serializer = serde_json::pretty(&mut writer);
    // Perform any specific assertions on serializer if needed
    assert!(!writer.output.is_empty()); // Ensure something is written
}

#[test]
fn test_pretty_large_input() {
    let mut writer = TestWriter { output: Vec::new() };
    let large_data = serde_json::json!({
        "key1": "value1",
        "key2": [1, 2, 3],
        "key3": {"nested_key": "nested_value"},
    });
    let serializer = serde_json::pretty(&mut writer);
    // Serialize the large data structure with the pretty serializer
    let _ = serializer.serialize(&large_data).unwrap();
    // Check that the output contains the serialized data
    assert!(String::from_utf8_lossy(&writer.output).contains("key1"));
}

#[should_panic]
#[test]
fn test_pretty_with_failing_writer() {
    struct FailingWriter;

    impl std::io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "write error"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = FailingWriter;
    let serializer = serde_json::pretty(&mut writer);
    // Attempt to serialize will panic due to write error
    let _ = serializer.serialize(&serde_json::json!(null)).unwrap();
}

#[test]
fn test_pretty_with_special_characters() {
    let mut writer = TestWriter { output: Vec::new() };
    let special_data = serde_json::json!({
        "string": "Hello, world! \n\t\r\"Special Characters\"",
        "number": 1234,
    });
    let serializer = serde_json::pretty(&mut writer);
    let _ = serializer.serialize(&special_data).unwrap();
    assert!(String::from_utf8_lossy(&writer.output).contains("Special Characters"));
}


