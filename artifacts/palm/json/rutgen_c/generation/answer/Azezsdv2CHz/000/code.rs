// Answer 0

#[test]
fn test_end_object_without_value() {
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
        current_indent: 1,
        has_value: false,
        indent: b"  ",
    };

    formatter.end_object(&mut writer).unwrap();
    assert_eq!(writer.output, b"}");
}

#[test]
fn test_end_object_with_value() {
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
        current_indent: 1,
        has_value: true,
        indent: b"  ",
    };

    formatter.end_object(&mut writer).unwrap();
    assert_eq!(writer.output, b"\n}");
}

#[test]
fn test_end_object_with_multiple_indents() {
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
        current_indent: 2,
        has_value: true,
        indent: b"  ",
    };

    formatter.end_object(&mut writer).unwrap();
    assert_eq!(writer.output, b"\n  }");
}

