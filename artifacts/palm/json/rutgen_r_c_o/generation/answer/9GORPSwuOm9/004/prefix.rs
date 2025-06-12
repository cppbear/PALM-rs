// Answer 0

#[test]
fn test_begin_object_key_first_false_with_indent() {
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

    let writer = &mut MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 5,
        has_value: false,
        indent: b"  ",
    };

    let _ = formatter.begin_object_key(writer, false);
}

#[test]
fn test_begin_object_key_first_false_with_zero_indent() {
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

    let writer = &mut MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    let _ = formatter.begin_object_key(writer, false);
}

#[test]
fn test_begin_object_key_first_false_with_large_indent() {
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

    let writer = &mut MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 100,
        has_value: false,
        indent: b"  ",
    };

    let _ = formatter.begin_object_key(writer, false);
}

#[test]
fn test_begin_object_key_first_false_with_max_indent_length() {
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

    let writer = &mut MockWriter { output: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 50,
        has_value: false,
        indent: b"1234567890123456", // Length of 16
    };

    let _ = formatter.begin_object_key(writer, false);
}

