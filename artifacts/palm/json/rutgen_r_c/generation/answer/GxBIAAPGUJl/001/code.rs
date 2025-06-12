// Answer 0

#[test]
fn test_begin_array() {
    use std::io::Cursor;

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

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    // Test that the function writes the correct output
    assert!(formatter.begin_array(&mut writer).is_ok());
    assert_eq!(writer.buffer, b"[");

    // Test that current_indent is updated correctly
    assert_eq!(formatter.current_indent, 1);
    assert!(!formatter.has_value);
}

#[test]
fn test_begin_array_multiple_calls() {
    use std::io::Cursor;

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

    let mut writer = TestWriter { buffer: Vec::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    assert!(formatter.begin_array(&mut writer).is_ok());
    assert!(formatter.begin_array(&mut writer).is_ok());

    assert_eq!(writer.buffer, b"[[");
    assert_eq!(formatter.current_indent, 2);
}

#[test]
fn test_begin_array_with_empty_writer() {
    use std::io::Cursor;

    struct EmptyWriter;

    impl io::Write for EmptyWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0) // simulate no bytes written
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = EmptyWriter;
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    assert!(formatter.begin_array(&mut writer).is_ok());
    // Current indent should still be updated
    assert_eq!(formatter.current_indent, 1);
}

#[test]
fn test_begin_array_once() {
    use std::io::Cursor;

    struct StringWriter {
        buffer: String,
    }

    impl io::Write for StringWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.push_str(std::str::from_utf8(buf).unwrap());
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = StringWriter { buffer: String::new() };
    let mut formatter = PrettyFormatter {
        current_indent: 0,
        has_value: false,
        indent: b"  ",
    };

    assert!(formatter.begin_array(&mut writer).is_ok());
    assert_eq!(writer.buffer, "[");
    assert_eq!(formatter.current_indent, 1);
}


