// Answer 0

#[test]
fn test_begin_object_value() {
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
        indent: b"    ",
    };

    let result = formatter.begin_object_value(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.output, b": ");
}

#[test]
fn test_begin_object_value_empty_writer() {
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
        indent: b"    ",
    };

    let result = formatter.begin_object_value(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.output, b": ");
}

