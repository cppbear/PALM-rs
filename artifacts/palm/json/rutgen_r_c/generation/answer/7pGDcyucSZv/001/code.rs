// Answer 0

#[test]
fn test_end_object_value() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.end_object_value(&mut writer);

    assert!(result.is_ok());
    assert!(formatter.has_value);
}

#[test]
fn test_end_object_value_no_panics() {
    struct TestWriter {
        output: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = TestWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.end_object_value(&mut writer);

    assert!(result.is_ok());
    assert!(!formatter.has_value); // Check if has_value was correctly set only in the first call
    let result_2 = formatter.end_object_value(&mut writer);
    assert!(result_2.is_ok());
    assert!(formatter.has_value); // Check if has_value is true after the second call
}

