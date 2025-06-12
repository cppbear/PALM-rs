// Answer 0

#[test]
fn test_to_writer_serializes_string() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let value = "Hello, World!";
    let result = to_writer(&mut writer, &value);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"\"Hello, World!\"");
}

#[test]
fn test_to_writer_serializes_vector() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let value = vec![1, 2, 3];
    let result = to_writer(&mut writer, &value);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"[1,2,3]");
}

#[test]
fn test_to_writer_serializes_map() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let value = [("key1", 1), ("key2", 2)].iter().cloned().collect::<std::collections::HashMap<_, _>>();
    let result = to_writer(&mut writer, &value);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"{\"key1\":1,\"key2\":2}");
}

#[should_panic]
#[test]
fn test_to_writer_fails_on_non_string_keys() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    
    let mut writer = TestWriter { output: Vec::new() };
    let value = [(1, "value1"), (2, "value2")].iter().cloned().collect::<std::collections::HashMap<_, _>>();
    let _ = to_writer(&mut writer, &value);
}

