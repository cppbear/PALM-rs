// Answer 0

#[test]
fn test_begin_object() {
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
        indent: b"    ",
    };

    formatter.begin_object(&mut writer).unwrap();

    assert_eq!(writer.buffer, b"{");
    assert_eq!(formatter.current_indent, 1);
    assert_eq!(formatter.has_value, false);
}

#[test]
fn test_begin_object_multiple_calls() {
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
        indent: b"    ",
    };

    formatter.begin_object(&mut writer).unwrap();
    formatter.begin_object(&mut writer).unwrap();

    assert_eq!(writer.buffer, b"{{");
    assert_eq!(formatter.current_indent, 2);
    assert_eq!(formatter.has_value, false);
}

