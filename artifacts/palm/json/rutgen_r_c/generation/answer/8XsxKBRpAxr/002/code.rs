// Answer 0

fn test_end_array_error() {
    struct TestWriter {
        buffer: Vec<u8>,
        should_fail: bool,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            if self.should_fail {
                return Err(io::Error::new(io::ErrorKind::Other, "Write failed"));
            }
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let indent_bytes = b"    ";
    let initial_indent = 1;

    // Case where we set self.has_value to true, and failure will occur during indent
    let mut formatter = PrettyFormatter {
        current_indent: initial_indent,
        has_value: true,
        indent: indent_bytes,
    };

    let mut writer = TestWriter {
        buffer: Vec::new(),
        should_fail: true, // trigger an error in indent
    };

    let result = formatter.end_array(&mut writer);

    assert!(result.is_err()); // Should return an error
    assert_eq!(writer.buffer.len(), 0); // No data should be written
}

fn test_end_array_success() {
    struct TestWriter {
        buffer: Vec<u8>,
        should_fail: bool,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let indent_bytes = b"    ";
    let initial_indent = 1;

    // Case where we set self.has_value to true, and no errors will occur
    let mut formatter = PrettyFormatter {
        current_indent: initial_indent,
        has_value: true,
        indent: indent_bytes,
    };

    let mut writer = TestWriter {
        buffer: Vec::new(),
        should_fail: false, // No failure this time
    };

    let result = formatter.end_array(&mut writer);

    assert!(result.is_ok()); // Should return Ok
    assert_eq!(writer.buffer.as_slice(), b"\n]"); // Correct output after formatting
}

fn test_end_array_no_value() {
    struct TestWriter {
        buffer: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let indent_bytes = b"    ";
    let initial_indent = 1;

    // Case where self.has_value is false
    let mut formatter = PrettyFormatter {
        current_indent: initial_indent,
        has_value: false,
        indent: indent_bytes,
    };

    let mut writer = TestWriter {
        buffer: Vec::new(),
    };

    let result = formatter.end_array(&mut writer);

    assert!(result.is_ok()); // Should return Ok
    assert_eq!(writer.buffer.as_slice(), b"]"); // Expected output only closing bracket
}

