// Answer 0

#[test]
fn test_serialize_str_valid_input() {
    struct MockWriter {
        output: String,
    }

    struct MockFormatter;

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write(&mut self, data: &str) -> std::io::Result<usize> {
            self.output.push_str(data);
            Ok(data.len())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;

    let result = serialize_str(&mut writer, &formatter, "hello");
    assert!(result.is_ok());
    assert_eq!(writer.output, "\"hello\"");
}

#[test]
fn test_serialize_str_empty_input() {
    struct MockWriter {
        output: String,
    }

    struct MockFormatter;

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write(&mut self, data: &str) -> std::io::Result<usize> {
            self.output.push_str(data);
            Ok(data.len())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;

    let result = serialize_str(&mut writer, &formatter, "");
    assert!(result.is_ok());
    assert_eq!(writer.output, "\"\"");
}

#[test]
#[should_panic]
fn test_serialize_str_panics_on_writer_error() {
    struct MockWriter;

    struct MockFormatter;

    impl MockWriter {
        fn write(&mut self, _data: &str) -> std::io::Result<usize> {
            Err(std::io::Error::new(std::io::ErrorKind::Other, "mock error"))
        }
    }

    let mut writer = MockWriter;
    let formatter = MockFormatter;

    let _result = serialize_str(&mut writer, &formatter, "test");
}

#[test]
fn test_serialize_str_special_chars() {
    struct MockWriter {
        output: String,
    }

    struct MockFormatter;

    impl MockWriter {
        fn new() -> Self {
            MockWriter {
                output: String::new(),
            }
        }

        fn write(&mut self, data: &str) -> std::io::Result<usize> {
            self.output.push_str(data);
            Ok(data.len())
        }
    }

    let mut writer = MockWriter::new();
    let formatter = MockFormatter;

    let result = serialize_str(&mut writer, &formatter, "line1\nline2\t\"quotes\"");
    assert!(result.is_ok());
    assert_eq!(writer.output, "\"line1\\nline2\\t\\\"quotes\\\"\"");
}

