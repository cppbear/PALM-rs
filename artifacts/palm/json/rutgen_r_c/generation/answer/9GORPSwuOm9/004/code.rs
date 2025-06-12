// Answer 0

#[test]
fn test_begin_object_key_non_first() {
    struct MockWriter {
        content: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.content.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { content: Vec::new() };
    let indent: &[u8] = b"  "; // Example indentation
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent,
    };

    // Call the function under test with first set to false
    let result = formatter.begin_object_key(&mut writer, false);

    // Assert the result
    assert!(result.is_ok());
    assert_eq!(writer.content, b",\n  ");
}

#[test]
fn test_begin_object_key_first() {
    struct MockWriter {
        content: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.content.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mut writer = MockWriter { content: Vec::new() };
    let indent: &[u8] = b"  "; // Example indentation
    let mut formatter = PrettyFormatter {
        current_indent: 2,
        has_value: false,
        indent,
    };

    // Call the function under test with first set to true
    let result = formatter.begin_object_key(&mut writer, true);

    // Assert the result
    assert!(result.is_ok());
    assert_eq!(writer.content, b"\n  ");
}

