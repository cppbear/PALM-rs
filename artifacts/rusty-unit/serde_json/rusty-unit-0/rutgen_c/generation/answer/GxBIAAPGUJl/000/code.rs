// Answer 0

#[test]
fn test_begin_array_increments_indent() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.begin_array(&mut writer);
    assert!(result.is_ok());
    assert_eq!(formatter.current_indent, 1);
    assert_eq!(writer.output, b"[");
}

#[test]
fn test_begin_array_first_value() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let result = formatter.begin_array(&mut writer);
    assert!(result.is_ok());
    assert_eq!(formatter.current_indent, 1);
    assert_eq!(writer.output, b"[");
}

#[test]
fn test_begin_array_no_value() {
    struct MockWriter {
        output: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.output.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    formatter.begin_array(&mut writer).unwrap();
    let result = formatter.end_array(&mut writer);
    assert!(result.is_ok());
    assert_eq!(writer.output, b"[]");
}

